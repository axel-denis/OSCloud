use std::sync::Arc;

use axum::extract::{Multipart, Path, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use tower::ServiceExt;

use crate::database::model::User;
use crate::utils::users::verifiy_user_path;
use crate::AppState;

use tower_http::services::ServeDir;

pub async fn upload(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    mut multipart: Multipart,
) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
}

pub async fn download(
    Extension(local_user): Extension<User>,
    State(app_state): State<Arc<AppState>>,
    Path(file): Path<String>,
    request: Request,
) -> impl IntoResponse {
    let path = match verifiy_user_path(&app_state.userdata, &file, local_user) {
        Some(path) => path,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };
    if !path.path().exists() || !path.path().is_file() {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let service = ServeDir::new(path.path());
    match service.oneshot(request).await {
        Ok(result) => result.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
