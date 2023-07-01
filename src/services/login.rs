use crate::users::{self, User};
use actix_web::{web, HttpResponse, Responder};
use chrono;
use jsonwebtoken;
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

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    pub sub: i64,
    pub exp: usize,
}

fn get_secret() -> Vec<u8> {
    std::env::var("ACCESS_TOKEN_SECRET")
        .expect("ACCESS_TOKEN_SECRET must be set.")
        .into_bytes()
}

fn create_jwt(user: &User) -> Result<LoginResponse, jsonwebtoken::errors::Error> {
    let expiration_time = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(1800))
        .expect("invalid timestamp")
        .timestamp();
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
    let claims = Claims {
        sub: user.id,
        exp: expiration_time as usize,
    };
    let key = jsonwebtoken::EncodingKey::from_secret(&get_secret());
    let token = jsonwebtoken::encode(&header, &claims, &key)?;

    Ok(LoginResponse { token })
}

pub async fn login(login: web::Json<LoginRequest>) -> impl Responder {
    match users::get_user(&login.name, &login.password) {
        Err((code, error)) => HttpResponse::build(code).body(error.to_string()),
        Ok(user) => {
            match create_jwt(&user) {
                Ok(token) => HttpResponse::Ok().json(token),
                Err(error) => {
                    HttpResponse::InternalServerError().body(error.to_string()) // to ofuscate
                }
            }
        }
    }
}
