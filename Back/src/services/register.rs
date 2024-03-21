use crate::database::model::Role;
use crate::{jwt_manager, AppState};
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    #[serde(rename = "jwt")]
    pub token: String,
}

enum RegisterOutcome {
    Connected(String),
    AlreadyExist,
    Error,
}

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(register): Json<RegisterRequest>,
) -> Response {
    let outcome: RegisterOutcome =
        match app_state
            .userdata
            .create_user(&register.name, &register.password, Role::User, false)
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
