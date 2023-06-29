use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize};
use crate::users;

#[derive(Debug, Deserialize)]
pub struct Loggin {
    name: String,
    password: String,
}

pub async fn login(loggin: web::Json<Loggin>) -> impl Responder {
    match users::get_user(&loggin.name, &loggin.password) {
        Ok(user) => {
            HttpResponse::Ok().body(format!("{:?}, Logged!", user))
        },
        Err((code, error)) => {
            HttpResponse::build(code).body(format!("{}", error))
        }
    }
}
