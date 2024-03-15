// use crate::{
//     database::{model::Role, UserData},
//     jwt_manager::encode_jwt,
// };
// use actix_web::{web, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};

use crate::database::{model::Role, UserData};
use crate::{jwt_manager, AppState};
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

// pub async fn _register_not_opti(
//     db: web::Data<UserData>,
//     register: web::Json<RegisterRequest>,
// ) -> impl Responder {
//     let user = db.create_user(&register.name, &register.password, Role::User);

//     match user {
//         Err(err) => {
//             if err.is::<std::io::Error>() {
//                 HttpResponse::Conflict().body("User already exist")
//             } else {
//                 HttpResponse::InternalServerError().finish()
//             }
//         }
//         Ok(user) => match encode_jwt(&user) {
//             Ok(token) => HttpResponse::Ok().json(RegisterResponse { token }),
//             Err(_) => HttpResponse::InternalServerError().finish(),
//         },
//     }
// }

enum RegisterOutcome {
    Connected(String),
    AlreadyExist,
    Error,
}

// pub async fn register(
//     db: web::Data<UserData>,
//     register: web::Json<RegisterRequest>,
// ) -> impl Responder {
//     let outcome: RegisterOutcome = web::block(move || {
//         match db.create_user(&register.name, &register.password, Role::User) {
//             Err(err) => {
//                 if err.is::<std::io::Error>() {
//                     RegisterOutcome::AlreadyExist
//                 } else {
//                     RegisterOutcome::Error
//                 }
//             }
//             Ok(user) => match encode_jwt(&user) {
//                 Ok(token) => RegisterOutcome::Connected(token),
//                 Err(_) => RegisterOutcome::Error,
//             },
//         }
//     }).await.unwrap_or(RegisterOutcome::Error);
//     match outcome {
//         RegisterOutcome::Connected(token) => HttpResponse::Ok().json(RegisterResponse { token }),
//         RegisterOutcome::AlreadyExist => HttpResponse::Conflict().body("User already exist"),
//         _ => HttpResponse::InternalServerError().finish()
//     }
// }

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(register): Json<RegisterRequest>,
) -> Response {
    let outcome: RegisterOutcome =
        match app_state
            .userdata
            .create_user(&register.name, &register.password, Role::User)
        {
            Err(err) => {
                if err.is::<std::io::Error>() {
                    RegisterOutcome::AlreadyExist
                } else {
                    RegisterOutcome::Error
                }
            }
            Ok(user) => match jwt_manager::encode_jwt(&user) {
                Ok(token) => RegisterOutcome::Connected(token),
                Err(_) => RegisterOutcome::Error,
            },
        };
    match outcome {
        RegisterOutcome::Connected(token) => {
            (StatusCode::OK, axum::Json(RegisterResponse { token })).into_response()
        }
        RegisterOutcome::AlreadyExist => {
            (StatusCode::CONFLICT, "User already exists").into_response()
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
