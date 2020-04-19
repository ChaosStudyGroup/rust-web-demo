use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{Error, web};
use actix_web::dev::{HttpServiceFactory, ServiceRequest, ServiceResponse};
use futures::Future;
use futures::future::{ok, Ready};

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
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AccessLogMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AccessLogMiddleware { service })
    }
}

pub struct AccessLogMiddleware<S> {
    service: S,
}

impl<S, B> Service for AccessLogMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let begin = std::time::SystemTime::now();

        // request information
        let path = req.path().to_string();
        let method = req.method().as_str().to_string();
        let ip_addr = req.connection_info().remote().unwrap().to_string();
        let queries = req.query_string().to_string();

        // Todo: Request body is necessary.

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            // Todo: Response body is necessary.

            let duration = begin.elapsed().unwrap().as_millis();

            log::info!("path: {}, method: {}, ip: {}, queries: {}, duration: {}ms",
                     path,
                     method,
                     ip_addr,
                     queries,
                     duration
            );
            Ok(res)
        })
    }
}