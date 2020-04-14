#[macro_use]
extern crate validator_derive;

use std::error::Error;
use actix_web::{App, HttpServer, web};

use crate::utility::{db, log};
use crate::handler::{user, asset};

mod conf;
mod handler;
mod utility;


#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化mysql连接池
    db::mysql::init_pool(db::mysql::create_pool().await);

    // 初始化redis连接池
    db::redis::init_pool(db::redis::create_pool());

    // 初始化日志文件
    log::init_log();

    // 创建app
    let app_factory = || {
        App::new().service(web::scope("/api")
            .route("/user/login", web::post().to(user::login))
        ).service(web::scope("/static")
            .route("/index.html", web::get().to(asset::index))
        )
    };

    // 运行服务，绑定监听端口
    HttpServer::new(app_factory).bind(conf::global().addr())?.run().await?;

    Ok(())
}
