use chrono::NaiveDateTime;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::row::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    pub id: u32,
    pub username: String,
    pub age: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserModel {
    pub fn new(id: u32, username: String) -> Self {
        UserModel {
            id,
            username,
            age: 0,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }

    pub fn table() -> &'static str {
        "t_users"
    }
}

impl Default for UserModel {
    fn default() -> Self {
        UserModel {
            id: 0,
            username: "".to_string(),
            age: 0,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }
}

impl<'c> sqlx::FromRow<'c, MySqlRow<'c>> for UserModel {
    fn from_row(row: &MySqlRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(UserModel {
            id: row.get("id"),
            username: row.get("username"),
            age: row.get("age"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}
