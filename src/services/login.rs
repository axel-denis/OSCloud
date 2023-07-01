use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::users::get_user;
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
    match get_user(&login.name, &login.password) {
        Err((code, error)) => HttpResponse::build(code).body(error.to_string()),
        Ok(user) => {
            match encode_jwt(&user) {
                Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
                Err(error) => {
                    HttpResponse::InternalServerError().body(error.to_string()) // to ofuscate
                }
            }
        }
    }
}
