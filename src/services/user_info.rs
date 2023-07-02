use crate::{
    jwt_manager::decode_jwt,
    users::{get_user_from_id, get_user_from_name, Type, User},
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserInfoRequest {
    name: String,
}

#[derive(Serialize)]
pub struct UserInfoResponse {
    #[serde(rename = "jwt")]
    pub token: String,
}

pub async fn user_info(req: HttpRequest, user_info: web::Json<UserInfoRequest>) -> impl Responder {
    if let Some(local_user) = req.extensions().get::<User>() {
        match get_user_from_name(&user_info.name) {
            Err((code, _)) => HttpResponse::build(code).finish(),
            Ok(user) => {
                if user.id == local_user.id || local_user.user_type == Type::Admin {
                    HttpResponse::Ok().json(user)
                } else {
                    HttpResponse::Unauthorized().finish()
                }
            }
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
