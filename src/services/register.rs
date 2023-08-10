use crate::{
    database::{model::Role, UserData},
    jwt_manager::encode_jwt,
};
use actix_web::{web, HttpResponse, Responder};
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

pub async fn register(
    db: web::Data<UserData>,
    register: web::Json<RegisterRequest>,
) -> impl Responder {
    match db.create_user(&register.name, &register.password, Role::User) {
        Err(err) => {
            if err.is::<std::io::Error>() {
                HttpResponse::Conflict().body("User already exist")
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
        Ok(user) => match encode_jwt(&user) {
            Ok(token) => HttpResponse::Ok().json(RegisterResponse { token }),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
    }
}
