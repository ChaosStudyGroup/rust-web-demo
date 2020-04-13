use std::fmt::{Display, Formatter};
use actix_web::HttpResponse;
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct RespError {
    code: u32,
    msg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    tip: String,
}

#[derive(Debug, Serialize)]
pub struct RespData<T> {
    code: u32,
    msg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    tip: String,
    data: T,
}

#[derive(Debug, Serialize)]
enum ErrorKind {
    System = 1001,

    ParamsInvalid = 2001,
    NoPermission = 2002,
    IllegalOperation = 2003,

    UserNotFound = 3001,
    UserExists = 3002,
    PasswordInvalid = 3003,
}

impl RespError {
    #[allow(dead_code)]
    pub fn json(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }

    #[allow(dead_code)]
    pub fn data<T: Serialize>(self, data: T) -> RespData<T> {
        RespData{
            code: self.code,
            msg: self.msg,
            tip: self.tip,
            data,
        }
    }
}

impl<T: Serialize> RespData<T> {
    #[allow(dead_code)]
    pub fn json(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl Error for RespError {}

impl Display for RespError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Response Error: {:?}", self)
    }
}

macro_rules! impl_code_msg {
    ($name: ident, $code: expr, $msg: expr) => {
        #[allow(dead_code)]
        pub fn $name(tip: impl AsRef<str>, err: Option<&dyn Error>) -> RespError {
            if let Some(e) = err {
                log::error!("{}", e);
            }
            RespError{code: $code as u32, msg: $msg.into(), tip: tip.as_ref().into()}
        }
    }
}

impl_code_msg!(system, ErrorKind::System, "System error, please try later again.");
impl_code_msg!(params_invalid, ErrorKind::ParamsInvalid, "Params invalid, please check inputs.");
impl_code_msg!(no_permission, ErrorKind::NoPermission, "No permission, do you login success?");
impl_code_msg!(illegal_operation, ErrorKind::IllegalOperation, "Illegal operation.");
impl_code_msg!(user_not_found, ErrorKind::UserNotFound, "User not found.");
impl_code_msg!(user_exists, ErrorKind::UserExists, "User already exists.");
impl_code_msg!(password_invalid, ErrorKind::PasswordInvalid, "Account or password invalid, please check it.");
