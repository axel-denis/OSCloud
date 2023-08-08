use crate::{jwt_manager::encode_jwt, database::UserData};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use actix_web::http;
use std::io::{Error, ErrorKind};
use actix_web::http::StatusCode;
use bcrypt::verify;
use crate::database::model::User;

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

pub fn get_user_with_password(
    db: &UserData,
    username: &str,
    password: &str,
) -> Result<User, (http::StatusCode, Error)> {
    match db.get_user_by_name(username) {
        None => {Err((StatusCode::UNAUTHORIZED, Error::new(ErrorKind::NotFound,"User doesn't exist!")))}
        Some(user) => {
            match verify(password, &*user.password) {
                Ok(true) => {Ok(user)}
                Ok(false) => {Err((StatusCode::UNAUTHORIZED, Error::new(ErrorKind::PermissionDenied,"Wrong password!")))}
                Err(_) => {Err((StatusCode::INTERNAL_SERVER_ERROR, Error::new(ErrorKind::InvalidInput,"Invalid hash!")))}
            }
        }
    }
}

pub async fn login(db_pool: web::Data<UserData>, login: web::Json<LoginRequest>) -> impl Responder {
    match get_user_with_password(db_pool.get_ref(), &login.name, &login.password) {
        Err((code, _)) => HttpResponse::build(code).finish(),  // to ofuscate
        Ok(user) => {
            match encode_jwt(&user) {
                Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
                Err(_) => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}
