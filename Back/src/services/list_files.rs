use std::fs;
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum::{extract::Path, http::StatusCode};

use crate::database::model::User;
use crate::utils::files;
use crate::utils::users::path_in_user_mounts_points;
use crate::AppState;

// the given path is relative starting at the user mount point
pub async fn list_files(
    Path(dir): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
) -> Response {
    // TODO check that user has access to file

    let user_path = match path_in_user_mounts_points(&app_state.userdata, &dir, local_user) {
        Some(path) => path,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };
    match files::list_files::list_files(&user_path) {
        Ok(list) => (StatusCode::OK, axum::Json(list)).into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}
