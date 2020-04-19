use sqlx::prelude::*;

use crate::model::UserModel;
use crate::utility::db;

pub async fn find_by_username(username: &str) -> Result<Option<UserModel>, Box<dyn std::error::Error>> {
    let pool = match db::mysql::get_pool() {
        Some(p) => p,
        None => return Err("mysql get pool failed".into()),
    };

    let sql = format!("select * from {} where username = ?", UserModel::table());

    let user = match sqlx::query_as(&sql).bind(username).fetch_one(pool).await {
        Ok(u) => Ok(Some(u)),
        Err(e) => match e {
            sqlx::Error::RowNotFound => Ok(None),
            _ => Err(e.into())
        },
    };

    user
}