use serde::Deserialize;
use validator::Validate;
use actix_web::{Responder, web};
use crate::utility::result::*;

#[derive(Debug, Validate, Deserialize)]
pub struct LoginInput {
    #[validate(length(max = 10, message="username must be less than 10 chars."))]
    pub username: Option<String>,
    #[validate(length(min = 6, message="password must be more than 6 chars."))]
    pub password: Option<String>,
}

pub async fn login(input: web::Json<LoginInput>) -> impl Responder {
    if let Err(e) = input.validate() {
        return system("inputs invalid", Some(&e)).data(e).json();
    }

    Success::data(vec![1,2,3]).json()
}
