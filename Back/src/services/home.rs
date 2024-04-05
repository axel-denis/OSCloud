use crate::database::model::{ShareableUser, User};
use axum::extract::Json;
use axum::response::{IntoResponse, Response};
use axum::Extension;

pub async fn home(Extension(local_user): Extension<User>) -> Response {
    Json(ShareableUser::from(local_user)).into_response()
}
