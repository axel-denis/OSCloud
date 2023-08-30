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


pub async fn register_not_opti(
    db: web::Data<UserData>,
    register: web::Json<RegisterRequest>,
) -> impl Responder {
    let user = db.create_user(&register.name, &register.password, Role::User);

    match user {
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

pub async fn register(
    db: web::Data<UserData>,
    register: web::Json<RegisterRequest>,
) -> actix_web::Result<impl Responder> {
    let user = web::block(move || {
        db.create_user(&register.name, &register.password, Role::User)
    }).await.map_err(actix_web::error::ErrorInternalServerError)?;

    match user {
        Err(err) => {
            if err.is::<std::io::Error>() {
                Ok(HttpResponse::Conflict().body("User already exist"))
            } else {
                Err(actix_web::error::ErrorInternalServerError(501))
            }
        }
        Ok(user) => match encode_jwt(&user) {
            Ok(token) => Ok(HttpResponse::Ok().json(RegisterResponse { token })),
            Err(_) => Err(actix_web::error::ErrorInternalServerError(500)),
        },
    }
}
