use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Loggin {
    name: String,
    password: String,
}

pub async fn login(loggin: web::Json<Loggin>) -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}, Logged!", loggin))
}
