use crate::database::model::Role::Admin;
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
use std::sync::Arc;

pub async fn delete_user(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    Json(delete_info): Json<DeleteRequest>,
) -> StatusCode {
    if local_user.user_role != Admin && local_user.name != delete_info.name {
        StatusCode::UNAUTHORIZED
    } else {
        match app_state.userdata.delete_user(&delete_info.name) {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
