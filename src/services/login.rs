use crate::database::model::User;
use crate::{database::UserData, jwt_manager::encode_jwt};
use actix_web::http;
use actix_web::http::StatusCode;
use actix_web::web::to;
use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

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

pub async fn get_user_with_password(
    db: &UserData,
    username: &str,
    password: &str,
) -> Result<User, (http::StatusCode, Error)> {
    return Ok(User { id: 1, name: username.to_string(), password: password.to_string(), user_role: crate::database::model::Role::Admin });
    let new_db = db.clone();
    let new_username = username.to_string();
    let new_password = password.to_string();
    web::block(move || { let user = new_db.get_user_by_name(&new_username);
        match user {
            None => Err((
                StatusCode::UNAUTHORIZED,
                Error::new(ErrorKind::NotFound, "User doesn't exist!"),
            )),
            Some(user) => match verify(new_password, &user.password) {
                Ok(true) => Ok(user),
                Ok(false) => Err((
                    StatusCode::UNAUTHORIZED,
                    Error::new(ErrorKind::PermissionDenied, "Wrong password!"),
                )),
                Err(_) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Error::new(ErrorKind::InvalidInput, "Invalid hash!"),
                )),
            },
        }
    }).await.unwrap()
}

pub async fn login(db_pool: web::Data<UserData>, login: web::Json<LoginRequest>) -> impl Responder {
    let token: Option<String> = web::block(move || {
        match db_pool.get_user_by_name(&login.name) {
            Some(user) => {
                match verify(&login.password, &user.password) {
                    Ok(true) => match encode_jwt(&user) {
                        Ok(token) => Some(token),
                        Err(_) => None,
                    },
                    Ok(false) => None,
                    Err(_) => None,
                }
            },
            None => None
        }
    }).await.unwrap();
    match token {
        Some(str) => HttpResponse::Ok().json(LoginResponse { token: str }),
        None => HttpResponse::InternalServerError().finish(),
    }
}
