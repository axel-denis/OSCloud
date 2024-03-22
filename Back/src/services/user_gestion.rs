use crate::database::model::Role::Admin;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    name: String,
}

use crate::database::model::User;
use crate::AppState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::Extension;
use std::str::FromStr;
use std::sync::Arc;

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

#[derive(Debug, Deserialize)]
pub struct AddUserRequest {
    pub name: String,
    pub password: String,
    pub role: String,
}

// Admin
pub async fn add_user(
    State(app_state): State<Arc<AppState>>,
    Extension(_): Extension<User>,
    Json(register): Json<AddUserRequest>,
) -> Response {
    match app_state.userdata.create_user(
        &register.name,
        &register.password,
        match Role::from_str(&register.role) {
            Ok(role) => role,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Role should be admin or user").into_response()
            }
        },
        true,
    ) {
        Err(err) => {
            if err.is::<std::io::Error>() {
                (StatusCode::CONFLICT, "User already exists").into_response()
            } else {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
        Ok(_) => StatusCode::OK.into_response(),
    }
}

// Admin
// pub async fn enable_user(State)
