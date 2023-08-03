use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::database::model::{Role, User};
use crate::database::UserDatabase;

#[derive(Debug, Deserialize)]
pub struct UserInfoRequest {
    name: String,
}

#[derive(Serialize)]
pub struct UserInfoResponse {
    #[serde(rename = "jwt")]
    pub token: String,
}

pub async fn user_info(db: web::Data<UserDatabase>, req: HttpRequest, user_info: web::Json<UserInfoRequest>) -> impl Responder {
    if let Some(local_user) = req.extensions().get::<User>() {
        match db.get_user_by_name(&user_info.name) {
            None => HttpResponse::NotFound().finish(),
            Some(user) => {
                if user.id == local_user.id || local_user.user_role == Role::Admin {
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
