use std::{fs, io};
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;

use crate::database::model::{ShareType, User};
use crate::database::UserData;
use crate::utils::users::verifiy_user_path;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct FileShareRequest {
    file_path: String,
    share_type: ShareType,
    users_to_share_to: Vec<String>,
}

fn all_users_names_to_all_users(users: Vec<String>, db: &UserData) -> Result<Vec<User>, io::Error> {
    let mut result: Vec<User> = Vec::new();
    for name in users {
        if let Some(user) = db.get_user_by_name(&name) {
            result.push(user);
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "one user not found"))
        }
    };
    Ok(result)
}

fn share_to_user_list(users: Vec<User>, file_path: String, db: &UserData) {
}

pub async fn share_file(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    Json(req): Json<FileShareRequest>,
) -> Response {
    // checking that base path is file
    match verifiy_user_path(&app_state.userdata, &req.file_path, local_user) {
        Some(path) => {
            if path.path().exists() {
                if let Ok(users) = all_users_names_to_all_users(req.users_to_share_to, &app_state.userdata) {

                    StatusCode::OK.into_response()
                } else {
                (StatusCode::NOT_FOUND, "One or more user(s) not found").into_response()
                }
            } else {
                (StatusCode::NOT_FOUND, "file not found").into_response()
            }
        }
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}
