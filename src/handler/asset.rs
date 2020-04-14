use actix_web::{HttpRequest, Responder};
use crate::utility::result::*;

pub async fn index(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("");

    if name == "none" {
        return params_invalid("invalid name", None).json();
    }

    Success::json()
}
