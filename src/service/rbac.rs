use actix_web::HttpResponse;
use crate::dao::rbac as rbacDao;
use crate::model::UserModel;
use crate::utility::result::*;
use rand::prelude::*;
use sha1::Sha1;

pub async fn user_by_username(username: &str) -> Result<UserModel, HttpResponse> {
    let res = rbacDao::find_by_username(username).await.transpose();

    if res.is_none() {
        return Err(user_not_found("not found user", None).json());
    }

    res.unwrap().map_err(|e| {
        system("find user exception", Some(&*e)).json()
    })
}

pub async fn generate_auth_key(user_id: i32) -> Result<String, HttpResponse> {
    let mut rng = rand::thread_rng();
    let rs: u32 = rng.gen();

    let ts = chrono::Local::now().timestamp_subsec_millis();

    let crypt_str = format!("{}-{}", rs, ts);

    let mut sha1 = Sha1::new();
    sha1.update(crypt_str.as_bytes());
    let auth_key = sha1.digest().to_string();

    match rbacDao::update_with_auth_key(user_id, &auth_key).await {
        Ok(b) => {
            if b == false {
                return Err(system("mysql update failed", None).json());
            }
        },
        Err(e) => return Err(system("mysql exception", Some(&*e)).json()),
    };

    Ok(auth_key)
}

