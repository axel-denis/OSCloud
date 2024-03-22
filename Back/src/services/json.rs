use crate::database::model::{Role, User};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Extension;
use std::sync::Arc;

pub async fn save_to_json(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
) -> StatusCode {
    if local_user.user_role == Role::Admin {
        return match app_state.userdata.save_default() {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
    }
    StatusCode::UNAUTHORIZED
}

pub async fn import_from_json(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
) -> StatusCode {
    if local_user.user_role == Role::Admin {
        return match app_state.userdata.import_default() {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
    }
    StatusCode::UNAUTHORIZED
}
