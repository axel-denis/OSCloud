use crate::database::model::{Role, User};
// use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use crate::AppState;
use axum::extract::{Json, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct UserInfoRequest {
    name: String,
}

// pub async fn user_info(
//     db: web::Data<UserData>,
//     req: HttpRequest,
//     user_info: web::Json<UserInfoRequest>,
// ) -> impl Responder {
//     if let Some(local_user) = req.extensions().get::<User>() {
//         match db.get_user_by_name(&user_info.name) {
//             None => HttpResponse::NotFound().finish(),
//             Some(user) => {
//                 if user.id == local_user.id || local_user.user_role == Role::Admin {
//                     HttpResponse::Ok().json(user)
//                 } else {
//                     HttpResponse::Unauthorized().finish()
//                 }
//             }
//         }
//     } else {
//         HttpResponse::Unauthorized().finish()
//     }
// }

pub async fn user_info(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    Json(user_info): Json<UserInfoRequest>,
) -> Response {
    match app_state.userdata.get_user_by_name(&user_info.name) {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(user) => {
            if user.id == local_user.id || local_user.user_role == Role::Admin {
                (StatusCode::OK, axum::Json(user)).into_response()
            } else {
                StatusCode::UNAUTHORIZED.into_response()
            }
        }
    }
}
