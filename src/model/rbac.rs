use chrono::NaiveDateTime;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::row::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub auth_key: String,
    pub password_hash: String,
    pub password_reset_token: String,
    pub email: String,
    pub status: i16,
    pub created_at: i32,
    pub updated_at: i32,
}

impl UserModel {
    pub fn table() -> &'static str {
        "user"
    }
}

impl<'c> sqlx::FromRow<'c, MySqlRow<'c>> for UserModel {
    fn from_row(row: &MySqlRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(UserModel {
            id: row.get("id"),
            username: row.get("username"),
            auth_key: row.try_get("auth_key").unwrap_or_default(),
            password_hash: row.try_get("password_hash").unwrap_or_default(),
            password_reset_token: row.try_get("password_reset_token").unwrap_or_default(),
            email: row.get("email"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAssignmentModel {
    pub item_name: String,
    pub user_id: String,
    pub created_at: i32,
}

impl AuthAssignmentModel {
    pub fn table() -> &'static str {
        "auth_assignment"
    }
}

impl<'c> sqlx::FromRow<'c, MySqlRow<'c>> for AuthAssignmentModel {
    fn from_row(row: &MySqlRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(AuthAssignmentModel {
            item_name: row.get("item_name"),
            user_id: row.get("user_id"),
            created_at: row.get("created_at"),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthItemModel {
    pub name: String,
    pub r#type: i16,
    pub description: String,
    pub rule_name: String,
    pub data: String,
    pub created_at: i32,
    pub updated_at: i32,
}

impl AuthItemModel {
    pub fn table() -> &'static str {
        "auth_item"
    }
}

impl<'c> sqlx::FromRow<'c, MySqlRow<'c>> for AuthItemModel {
    fn from_row(row: &MySqlRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(AuthItemModel {
            name: row.get("name"),
            r#type: row.get("type"),
            description: row.get("description"),
            rule_name: row.get("rule_name"),
            data: row.get("data"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthItemChildModel {
    pub parent: String,
    pub child: String,
}

impl AuthItemChildModel {
    pub fn table() -> &'static str {
        "auth_item_child"
    }
}

impl<'c> sqlx::FromRow<'c, MySqlRow<'c>> for AuthItemChildModel {
    fn from_row(row: &MySqlRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(AuthItemChildModel {
            parent: row.get("parent"),
            child: row.get("child"),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRuleModel {
    pub name: String,
    pub data: String,
    pub created_at: i32,
    pub updated_at: i32,
}

impl AuthRuleModel {
    pub fn table() -> &'static str {
        "auth_rule"
    }
}

impl<'c> sqlx::FromRow<'c, MySqlRow<'c>> for AuthRuleModel {
    fn from_row(row: &MySqlRow<'c>) -> Result<Self, sqlx::Error> {
        Ok(AuthRuleModel {
            name: row.get("name"),
            data: row.get("data"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}
