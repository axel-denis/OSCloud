use axum::response::{IntoResponse, Response};
use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;

use crate::utils::files;
use crate::utils::files::list_files::FileInfo;

// the given path is relative starting at the user mount point
pub async fn list_files(Path(dir): Path<String>) -> Response {
    // TODO check that user has access to file
    match files::list_files::list_files(dir) {
        Ok(list) => (StatusCode::OK, axum::Json(list)).into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}
