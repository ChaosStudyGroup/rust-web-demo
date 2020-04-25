pub mod user;
pub mod asset;
pub mod middleware;

use actix_web::{web, dev::HttpServiceFactory};

pub fn api_routes() -> impl HttpServiceFactory {
    web::scope("/api")
        .route("/user/login", web::post().to(user::login))
}

pub fn static_routes() -> impl HttpServiceFactory {
    web::scope("/static")
        .route("/index.html", web::get().to(asset::index))
}
