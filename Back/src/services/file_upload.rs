use std::convert::Infallible;
use std::sync::Arc;

use axum::extract::{Multipart, Request, State};
use axum::response::Response;
use axum::{Extension, Json};
use serde::Serialize;

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

#[derive(Serialize)]
pub struct FileDownloadRequest {
    path: String,
}
use tower::util::ServiceExt;
pub async fn download(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
) -> ServeDir {
    ServeDir::new("assets")
}
