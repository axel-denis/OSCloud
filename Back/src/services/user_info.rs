use crate::database::model::{Role, ShareableUser, User, UserMountPoint};
use crate::AppState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct UserInfoRequest {
    name: String,
}
#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    user: ShareableUser,
    mount_points: Vec<String>,
}

pub async fn user_info(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    Json(user_info): Json<UserInfoRequest>,
) -> Response {
    match app_state.userdata.get_user_by_name(&user_info.name) {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(user) => match app_state.userdata.get_user_mounts_points(&local_user) {
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Some(mount_points) => {
                if user.id == local_user.id || local_user.user_role == Role::Admin {
                    (
                        StatusCode::OK,
                        axum::Json(UserInfoResponse {
                            user: ShareableUser {
                                id: user.id,
                                name: user.name,
                                user_role: user.user_role,
                                enabled: user.enabled,
                            },
                            mount_points: mount_points,
                        }),
                    )
                        .into_response()
                } else {
                    StatusCode::UNAUTHORIZED.into_response()
                }
            }
        },
    }
}

#[derive(Debug, Deserialize)]
pub struct UserEnablenessRequest {
    sort: String, // "none" | "enabled" | "disabled"
}

// Admin
pub async fn get_users_enableness(
    State(app_state): State<Arc<AppState>>,
    Extension(_): Extension<User>,
    Json(req): Json<UserEnablenessRequest>,
) -> Response {
    let all_users = match app_state.userdata.get_users_shareables() {
        Ok(users) => users,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    match req.sort.as_str() {
        "none" => (StatusCode::OK, axum::Json(all_users)).into_response(),
        "enabled" => (
            StatusCode::OK,
            axum::Json(
                all_users
                    .into_iter()
                    .filter(|usr| usr.enabled)
                    .collect::<Vec<ShareableUser>>(),
            ),
        )
            .into_response(),
        "disabled" => (
            StatusCode::OK,
            axum::Json(
                all_users
                    .into_iter()
                    .filter(|usr| !usr.enabled)
                    .collect::<Vec<ShareableUser>>(),
            ),
        )
            .into_response(),
        _ => StatusCode::BAD_REQUEST.into_response(),
    }
}
