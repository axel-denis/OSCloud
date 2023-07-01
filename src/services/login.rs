use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::users::get_user_with_password;
use crate::jwt_manager::encode_jwt;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    name: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    #[serde(rename = "jwt")]
    pub token: String,
}

pub async fn login(login: web::Json<LoginRequest>) -> impl Responder {
    match get_user_with_password(&login.name, &login.password) {
        Err((code, _)) => HttpResponse::build(code).finish(),
        Ok(user) => {
            match encode_jwt(&user) {
                Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
                Err(_) => {
                    HttpResponse::InternalServerError().finish() // to ofuscate
                }
            }
        }
    }
}
