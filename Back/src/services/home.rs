use crate::database::model::User;
// use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use axum::extract::Json;
use axum::response::{IntoResponse, Response};
use axum::Extension;

// use crate::database::model::User;
// use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};

// pub async fn home(req: HttpRequest) -> impl Responder {
//     if let Some(local_user) = req.extensions().get::<User>() {
//         HttpResponse::Ok().json(local_user)
//     } else {
//         HttpResponse::NotFound().finish()
//     }
// }

pub async fn home(Extension(local_user): Extension<User>) -> Response {
    Json(local_user).into_response()
}
