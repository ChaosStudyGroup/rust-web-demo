use actix_web::{Responder, web};
use crate::inputs::user::LoginInput;
use crate::utility::result::*;

pub async fn login(input: web::Json<LoginInput>) -> impl Responder {
    if let Err(e) = input.validate_error() {
        return e.json();
    }

    Success::data(vec![1,2,3]).json()
}
