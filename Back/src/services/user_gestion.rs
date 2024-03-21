use crate::database::model::Role::Admin;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    name: String,
}

use crate::database::model::User;
use crate::{jwt_manager, AppState};
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::Extension;
use std::sync::Arc;

use super::register::RegisterOutcome;
use super::register::RegisterRequest;
use super::register::RegisterResponse;
use crate::database::model::Role;

pub async fn delete_user(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    Json(delete_info): Json<DeleteRequest>,
) -> StatusCode {
    if local_user.user_role != Admin
    /* && local_user.name != delete_info.name */
    {
        // NOTE - self delete will be enabled when all data (file included) will be deleted
        StatusCode::UNAUTHORIZED
    } else {
        match app_state.userdata.delete_user(&delete_info.name) {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

pub async fn add_user(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    Json(register): Json<RegisterRequest>,
) -> Response {
    if local_user.user_role != Admin {
        StatusCode::UNAUTHORIZED.into_response()
    } else {
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
}
