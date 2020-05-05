pub mod rbac;
pub mod user;
pub mod middleware;

use actix_web::{web, dev::HttpServiceFactory};
use actix_files as fs;

pub fn api_routes() -> impl HttpServiceFactory {
    web::scope("/api")
        .route("/user/login", web::post().to(user::login))
}

pub fn static_routes() -> impl HttpServiceFactory {
    fs::Files::new("/admin", "./views")
}
