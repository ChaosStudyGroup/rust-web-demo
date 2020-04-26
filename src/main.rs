#[macro_use]
extern crate validator_derive;

#[macro_use]
mod utility;

use std::error::Error;
use actix_web::{App, HttpServer};

mod conf;
mod dao;
mod model;
mod handler;

use crate::utility::{db, log};

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
        App::new()
            // 自定义预处理中间件
            .wrap(handler::middleware::Log)
            // 自定义权限中间件
            .wrap(handler::middleware::Auth)
            // api相关的路由
            .service(handler::api_routes())
            // 静态资源相关的路由
            .service(handler::static_routes())
    };

    // 运行服务，绑定监听端口
    HttpServer::new(app_factory).bind(conf::global().addr())?.run().await?;

    Ok(())
}
