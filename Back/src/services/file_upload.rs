use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Multipart, Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use tower::ServiceExt;

use crate::database::model::User;
use crate::utils::users::{verifiy_user_path, VerifiedUserPath};
use crate::AppState;

use tower_http::services::ServeDir;

#[derive(TryFromMultipart)]
pub struct UploadFileRequest {
    file_path: String,
    file: Bytes,
}

pub async fn upload(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    data: TypedMultipart<UploadFileRequest>,
) -> Response {
    if let Some(path) = verifiy_user_path(&app_state.userdata, &data.file_path, local_user) {
        match tokio::fs::write(path.path(), &data.file).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

pub async fn download(
    Extension(local_user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    axum::extract::Path(file): axum::extract::Path<String>,
    request: Request,
) -> impl IntoResponse {
    if let Some(path) = verifiy_user_path(&app_state.userdata, &file, local_user) {
        if path.path().exists() && path.path().is_file() {
            return match ServeDir::new(path.path()).oneshot(request).await {
                Ok(result) => result.into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
            };
        }
    }
    return StatusCode::UNAUTHORIZED.into_response();
}