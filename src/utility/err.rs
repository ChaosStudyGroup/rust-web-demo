use serde::Serialize;
use actix_web::HttpResponse;

#[derive(Debug, Serialize)]
pub struct RespBody<T> {
    code: u32,
    msg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    tip: String,
    data: T,
}

impl<T: Serialize> RespBody<T> {
    pub fn json(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Debug, Serialize)]
pub struct Success {}

impl Success {
    pub fn json() -> HttpResponse {
        Self::data(Success {}).json()
    }

    pub fn data<T: Serialize>(data: T) -> RespBody<T> {
        RespBody {
            code: 1000,
            msg: "success".into(),
            tip: "".into(),
            data,
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    System = 1001,

    ParamsInvalid = 2001,
    NoPermission = 2002,
    IllegalOperation = 2003,

    UserNotFound = 3001,
    UserExists = 3002,
    PasswordInvalid = 3003,
}

#[derive(Debug)]
pub struct Error(ErrorKind, String, String);

impl Error {
    pub fn json(self) -> HttpResponse {
        self.data("").json()
    }

    pub fn data<T: Serialize>(self, data: T) -> RespBody<T> {
        RespBody {
            code: self.0 as u32,
            msg: self.1,
            tip: self.2,
            data,
        }
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "code: {:?}, msg: {}, tips: {}", self.0, self.1, self.2)
    }
}

macro_rules! impl_code_msg {
    ($name: ident, $code: expr, $msg: expr) => {
        #[allow(dead_code)]
        pub fn $name(tip: impl AsRef<str>, err: Option<&dyn std::error::Error>) -> Error {
            if let Some(e) = err {
                log::error!("{}", e);
            }
            Error($code, $msg.into(), tip.as_ref().into())
        }
    };
}

impl_code_msg!(system, ErrorKind::System, "System error, please try later again.");
impl_code_msg!(params_invalid, ErrorKind::ParamsInvalid, "Params invalid, please check inputs.");
impl_code_msg!(no_permission, ErrorKind::NoPermission, "No permission, do you login success?");
impl_code_msg!(illegal_operation, ErrorKind::IllegalOperation, "Illegal operation.");
impl_code_msg!(user_not_found, ErrorKind::UserNotFound, "User not found.");
impl_code_msg!(user_exists, ErrorKind::UserExists, "User already exists.");
impl_code_msg!(password_invalid, ErrorKind::PasswordInvalid, "Account or password invalid, please check it.");
