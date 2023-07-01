use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize,};
use crate::{users::{get_user_from_name, get_user_from_id, Type}, jwt_manager::decode_jwt};

#[derive(Debug, Deserialize)]
pub struct UserInfoRequest {
    name: String,
}

#[derive(Serialize)]
pub struct UserInfoResponse {
    #[serde(rename = "jwt")]
    pub token: String,
}

pub async fn user_info(auth: BearerAuth, user_info: web::Json<UserInfoRequest>) -> impl Responder {
    let local_user_id: i64 = decode_jwt(auth.token()).unwrap();
    let local_user = get_user_from_id(local_user_id).unwrap();

    match get_user_from_name(&user_info.name) {
        Err((code, _)) => HttpResponse::build(code).finish(),
        Ok(user) => {
            if user.id == local_user_id || local_user.user_type == Type::Admin {
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::Unauthorized().finish()
            }
        },
    }
}
