use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{Error, web, HttpMessage};
use actix_web::dev::{HttpServiceFactory, ServiceRequest, ServiceResponse};
use futures::{Future, StreamExt};
use futures::future::{ok, Ready};
use std::rc::Rc;
use std::cell::RefCell;
use bytes::BytesMut;
use actix_http::body::{ResponseBody, MessageBody};

pub mod user;
pub mod asset;


pub fn api_routes() -> impl HttpServiceFactory {
    web::scope("/api")
        .route("/user/login", web::post().to(user::login))
}

pub fn static_routes() -> impl HttpServiceFactory {
    web::scope("/static")
        .route("/index.html", web::get().to(asset::index))
}


// custom request log middleware
pub struct AccessLog;

impl<S, B> Transform<S> for AccessLog
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AccessLogMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AccessLogMiddleware {
            service: Rc::new(RefCell::new(service))
        })
    }
}

pub struct AccessLogMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AccessLogMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let begin = std::time::SystemTime::now();

        let mut svc = self.service.clone();

        Box::pin(async move {
            // request information
            let path = req.path().to_string();
            let method = req.method().as_str().to_string();
            let ip_addr = req.connection_info().remote().unwrap().to_string();
            let queries = req.query_string().to_string();

            // read request body
            let mut stream = req.take_payload();

            let mut body = BytesMut::new();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }
            let req_body = String::from_utf8(body.clone().to_vec()).unwrap();

            // put bytes back into request body
            let mut payload = actix_http::h1::Payload::empty();
            payload.unread_data(body.freeze());
            req.set_payload(payload.into());

            let mut resp = svc.call(req).await?;

            // read response body
            let mut stream = resp.take_body();

            let mut body = BytesMut::new();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }
            let resp_body = String::from_utf8(body.clone().to_vec()).unwrap();

            // put bytes back into response body
            let resp = resp.map_body(move |_, _| {
                ResponseBody::Body(body.into()).into_body()
            });

            let duration = begin.elapsed().unwrap().as_millis();

            log::info!("path: {}, method: {}, ip: {}, queries: {}, reqBody: {}, resBody: {}, duration: {}ms",
                     path,
                     method,
                     ip_addr,
                     queries,
                     req_body,
                     resp_body,
                     duration
            );
            Ok(resp)
        })
    }
}
