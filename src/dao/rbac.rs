use sqlx::prelude::*;

use crate::model::UserModel;
use crate::utility::db;

pub async fn find_by_username(username: &str) -> sql_result!(Option<UserModel>) {
    let sql = format!("select * from {} where username = ? limit 1", UserModel::table());

    sql_query_one!(sql, username)
}

pub async fn update_with_auth_key(user_id: i32, auth_key: &str) -> sql_result!(bool) {
    let sql = format!("update {} set auth_key = ? where id = ?", UserModel::table());

    sql_update!(sql, auth_key, user_id)
}
