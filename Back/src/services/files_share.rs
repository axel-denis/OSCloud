use std::sync::Arc;
use std::{fs, io};

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;

use crate::database::model::{FileShare, ShareType, User};
use crate::database::UserData;
use crate::utils::users::verifiy_user_path;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct FileShareRequest {
    file_path: String,
    share_type: ShareType,
    users_to_share_to: Vec<i32>,
}

fn all_users_ids_to_all_users(users: Vec<i32>, db: &UserData) -> Result<Vec<User>, io::Error> {
    let mut result: Vec<User> = Vec::new();
    for id in users {
        if let Some(user) = db.get_user_by_id(id) {
            result.push(user);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "one user not found",
            ));
        }
    }
    Ok(result)
}

fn share_to_user_list(users: &Vec<User>, share: &FileShare, db: &UserData) {
    //
}

pub async fn share_file(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    Json(req): Json<FileShareRequest>,
) -> Response {
    // checking that base path is file
    match verifiy_user_path(&app_state.userdata, &req.file_path, local_user.clone()) {
        Some(path) => {
            if path.path().exists() {
                if let Ok(share) =
                    app_state
                        .userdata
                        .add_file_share(&local_user, &path, req.share_type.clone())
                {
                    if req.share_type == ShareType::Public {
                        (StatusCode::OK, share.link).into_response()
                    } else {
                        if let Ok(users) =
                            all_users_ids_to_all_users(req.users_to_share_to, &app_state.userdata)
                        {
                            share_to_user_list(&users, &share, &app_state.userdata);
                            (StatusCode::OK, share.link).into_response()
                        } else {
                            (StatusCode::NOT_FOUND, "One or more user(s) not found").into_response()
                        }
                    }
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            } else {
                (StatusCode::NOT_FOUND, "file not found").into_response()
            }
        }
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}
