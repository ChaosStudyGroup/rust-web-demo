use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::conf;
use crate::service::rbac;
use crate::utility::context::Context;
use crate::utility::result::*;

#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequest {
    #[validate(length(max = 50, message = "username must be less than 50 chars."))]
    username: Option<String>,
    #[validate(length(min = 6, message = "password must be more than 6 chars."))]
    password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    user_id: i32,
    token: String,
}

pub async fn login(input: web::Json<LoginRequest>) -> HttpResponse {
    if let Err(e) = input.validate() {
        return system("inputs invalid", Some(&e)).data(e).json();
    }

    let user = match rbac::user_by_username(input.username.as_deref().unwrap()).await {
        Ok(u) => u,
        Err(e) => return e,
    };

    if user.password_hash != input.password.as_deref().unwrap() {
        return password_invalid("", None).json()
    }

    let token = match rbac::generate_auth_key(user.id).await {
        Ok(k) => k,
        Err(e) => return e,
    };

    Success::data(LoginResponse{user_id: user.id, token}).json()
}
