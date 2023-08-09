use crate::{jwt_manager::encode_jwt, database::{UserData, model::Role}};
use actix_web::{web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    name: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    #[serde(rename = "jwt")]
    pub token: String,
}

pub async fn register(db: web::Data<UserData>, register: web::Json<RegisterRequest>) -> impl Responder {
    match db.create_user(&register.name, &register.password, Role::User) {
        Err(err) => {
            if err.is::<std::io::Error>() {
                HttpResponse::Conflict().body("User already exist")
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
        Ok(user) => {
            return match encode_jwt(&user) {
                Ok(token) => HttpResponse::Ok().json(RegisterResponse { token }),
                Err(_) => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}
