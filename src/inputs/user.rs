use serde::{Deserialize};
use validator::{Validate, ValidationError, ValidationErrors};
use std::collections::HashMap;
use crate::utility::err::*;

#[derive(Debug, Validate, Deserialize)]
pub struct LoginInput {
    #[validate(length(max = 10, message="username must be less than 10 chars."))]
    pub username: Option<String>,
    #[validate(length(min = 6, message="password must be more than 6 chars."))]
    pub password: Option<String>,
}

impl LoginInput {
    pub fn validate_error(&self) -> Result<(), RespError> {
        if let Err(errs) = self.validate() {
            log::info!("info");
            log::warn!("warn");
            log::error!("error");
            return Err(system("some error happen", Some(&errs)));
        }

        Ok(())
    }
}
