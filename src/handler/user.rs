use actix_web::{HttpRequest, Responder, web, HttpResponse};
use crate::inputs::user::LoginInput;

pub async fn login(input: web::Json<LoginInput>) -> impl Responder {
    if let Err(e) = input.validate_error() {
        return e.data(1).json();
    }

    HttpResponse::Ok().body("ok")
}
