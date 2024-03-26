use std::sync::Arc;

use axum::extract::{Multipart, Path, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use tower::ServiceExt;

use crate::database::model::User;
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

pub async fn download(Path(file): Path<String>, request: Request) -> impl IntoResponse {
    // path is valid
    let path = std::path::Path::new(&file);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            StatusCode::BAD_REQUEST.into_response();
        }
    }

    if components.count() != 1 {
        return StatusCode::BAD_REQUEST.into_response();
    };
    //
    let service = ServeDir::new("assets");
    let result = service.oneshot(request).await;
    result.unwrap().into_response()
}
