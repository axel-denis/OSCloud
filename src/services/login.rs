use crate::{database::UserData, jwt_manager::encode_jwt};
use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use serde::{Deserialize, Serialize};

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

enum LoginOutcome {
    Connected(String),
    WrongPassword,
    UserNotFound,
    Error
}

pub async fn login(db_pool: web::Data<UserData>, login: web::Json<LoginRequest>) -> impl Responder {
    let outcome: LoginOutcome = web::block(move || {
        match db_pool.get_user_by_name(&login.name) {
            Some(user) => {
                match verify(&login.password, &user.password) {
                    Ok(true) => match encode_jwt(&user) {
                        Ok(token) => LoginOutcome::Connected(token),
                        Err(_) => LoginOutcome::Error,
                    },
                    Ok(false) => LoginOutcome::WrongPassword,
                    Err(_) => LoginOutcome::Error,
                }
            },
            None => LoginOutcome::UserNotFound,
        }
    }).await.unwrap_or(LoginOutcome::Error);

    match outcome {
        LoginOutcome::Connected(str) => HttpResponse::Ok().json(LoginResponse { token: str }),
        LoginOutcome::Error => HttpResponse::InternalServerError().finish(),
        _ => HttpResponse::Unauthorized().finish(),
    }
}
