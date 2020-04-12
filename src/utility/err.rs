use std::fmt::{Display, Formatter};
use actix_web::HttpResponse;
use serde::{Serialize, Serializer};
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
    pub fn json(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }

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
        pub fn $name(tip: impl AsRef<str>, err: Option<&dyn Error>) -> RespError {
            if let Some(e) = err {
                log::error!("{}", e);
            }
            RespError{code: $code as u32, msg: $msg.into(), tip: tip.as_ref().into()}
        }
    }
}

impl_code_msg!(system, ErrorKind::System, "System error, please try later again.");
