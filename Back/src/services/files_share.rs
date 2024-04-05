use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::database::model::{FileShare, ShareType, ShareableUser, User};
use crate::database::UserData;
use crate::utils::files::list_files::{FileInfo, FileType};
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
    for user in users.iter() {
        let _ = db.add_file_share_user(share, &user);
    }
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

pub struct OptionFileInfo {
    pub name: Option<OsString>,
    pub file_type: FileType,
}
pub async fn list_shared_to_me(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
) -> Response {
    match app_state.userdata.get_shared_to_user(&local_user) {
        Some(shared_to_me) => {
            let list: Vec<FileInfo> = shared_to_me
                .iter()
                .map(|f_share| {
                    let path = Path::new(&f_share.path);
                    OptionFileInfo {
                        name: match path.file_name().to_owned() {
                            Some(name) => Some(name.to_owned()),
                            None => None,
                        },
                        file_type: if path.is_file() {
                            FileType::File
                        } else if path.is_dir() {
                            FileType::Folder
                        } else {
                            FileType::Other
                        },
                    }
                })
                .filter_map(|opt| {
                    if opt.name.is_some() {
                        Some(FileInfo {
                            name: opt.name.unwrap(),
                            file_type: opt.file_type,
                        })
                    } else {
                        None
                    }
                })
                .collect();
            // NOTE - the filter discards all not valid elements (there shouldn't be)
            // but still they are not "properly" handled
            (StatusCode::OK, axum::Json(list)).into_response()
        }
        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Serialize)]
struct FileSharedInfoResponse {
    pub id: i32,
    pub owner: ShareableUser,
    pub path: String,
    pub share_type: ShareType,
    pub link: String,
    pub shared_to: Vec<ShareableUser>,
}

// Gives all the share related info of one file
// NOTE - only the owner can see the sharing info.
// Will maybe be changed in the futur if the user has shared access to the file
pub async fn file_shared_info(
    axum::extract::Path(path): axum::extract::Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
) -> Response {
    if let Some(verified_path) = verifiy_user_path(&app_state.userdata, &path, local_user) {
        match app_state.userdata.get_share_from_file_path(&verified_path.path()) {
            Some(list) => {
                let output: Vec<FileSharedInfoResponse> = list
                    .iter()
                    .filter_map(|elem| {
                        Some(FileSharedInfoResponse {
                            id: elem.id,
                            owner: if let Some(owner) =
                                app_state.userdata.get_user_by_id(elem.owner_user_id)
                            {
                                ShareableUser::from(owner)
                            } else {
                                return None;
                            },
                            path: elem.path.to_owned(),
                            share_type: elem.share_type.to_owned(),
                            link: elem.link.to_owned(),
                            shared_to: if let Some(users_id) =
                                app_state.userdata.get_file_users_shared_to_from_path(&verified_path.path())
                            {
                                users_id
                                    .iter()
                                    .filter_map(|id| {
                                        Some(ShareableUser::from(
                                            app_state.userdata.get_user_by_id(*id)?,
                                        ))
                                    })
                                    .collect()
                            } else {
                                return None;
                            },
                        })
                    })
                    .collect();
                (StatusCode::OK, axum::Json(output)).into_response()
            }
            None => StatusCode::NOT_FOUND.into_response(),
        }
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
