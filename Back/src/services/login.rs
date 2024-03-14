use bcrypt::verify;
use crate::{jwt_manager, AppState};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
    Error,
}

pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(login): Json<LoginRequest>,
) -> Response {
    let outcome: LoginOutcome = match app_state.userdata.get_user_by_name(&login.name) {
        Some(user) => match verify(&login.password, &user.password) {
            Ok(true) => match jwt_manager::encode_jwt(&user) {
                Ok(token) => LoginOutcome::Connected(token),
                Err(_) => LoginOutcome::Error,
            },
            Ok(false) => LoginOutcome::WrongPassword,
            Err(_) => LoginOutcome::Error,
        },
        None => LoginOutcome::UserNotFound,
    };

    match outcome {
        LoginOutcome::Connected(str) => {
            (StatusCode::OK, axum::Json(LoginResponse { token: str })).into_response()
        }
        LoginOutcome::Error => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        _ => StatusCode::UNAUTHORIZED.into_response(),
    }
}
