use crate::database::model::{Role, User};
// use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use crate::AppState;
use axum::extract::{Json, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use serde::Deserialize;
use std::sync::Arc;

// use crate::database::model::User;
// use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};

// pub async fn home(req: HttpRequest) -> impl Responder {
//     if let Some(local_user) = req.extensions().get::<User>() {
//         HttpResponse::Ok().json(local_user)
//     } else {
//         HttpResponse::NotFound().finish()
//     }
// }

pub async fn home(
    Extension(local_user): Extension<User>,
) -> Response {
    Json(local_user).into_response()
}
