use std::fs;
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{extract::Path, http::StatusCode};
use axum::{Extension, Json};
use serde::Deserialize;
use serde_json::to_string;

use crate::database::model::User;
use crate::utils::files;
use crate::utils::users::verifiy_user_path;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct FileRenameRequest {
    file_path: String,
    new_name: String,
}

pub async fn rename_file(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    Json(req): Json<FileRenameRequest>,
) -> Response {
    // checking that base path is file
    let base_path = if let Some(user_path) =
        verifiy_user_path(&app_state.userdata, &req.file_path, local_user.clone())
    {
        user_path
    } else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    //cheking that file name is valid & accessible
    let new_path = if let Some(np) = base_path.path().to_str() {
        if let Some(new_path) = verifiy_user_path(&app_state.userdata, &np.to_string(), local_user)
        {
            new_path
        } else {
            return (StatusCode::UNAUTHORIZED, "Innaccessible file name").into_response();
        }
    } else {
        return (StatusCode::BAD_REQUEST, "Invalid file name").into_response();
    };

    // Checking for conflicts
    if !base_path.path().exists() {
        return (StatusCode::NOT_FOUND, "Given file to rename does not exist").into_response();
    };
    if new_path.path().exists() {
        return (
            StatusCode::CONFLICT,
            "Given new file name already exists for another file",
        )
            .into_response();
    };

    match fs::rename(base_path.path(), new_path.path()) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
