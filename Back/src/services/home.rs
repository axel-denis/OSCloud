use crate::database::model::User;
// use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use axum::extract::Json;
use axum::response::{IntoResponse, Response};
use axum::Extension;

pub async fn home(Extension(local_user): Extension<User>) -> Response {
    Json(local_user).into_response()
}
