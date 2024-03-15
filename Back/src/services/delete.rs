use crate::database::model::Role::Admin;
// use crate::database::model::User;
// use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    name: String,
}

// pub async fn delete_user(
//     db: web::Data<UserData>,
//     req: HttpRequest,
//     delete_info: web::Json<DeleteRequest>,
// ) -> impl Responder {
//     if let Some(local_user) = req.extensions().get::<User>() {
//         if local_user.user_role != Admin && local_user.name != delete_info.name {
//             return HttpResponse::Unauthorized().body("Bad permission");
//         }

//         return match db.delete_user(&delete_info.name) {
//             Ok(_) => HttpResponse::Ok().body("User deleted"),
//             Err(_) => HttpResponse::Unauthorized().finish(),
//         };
//     }
//     HttpResponse::Unauthorized().finish()
// }

use crate::database::model::User;
use crate::AppState;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::Extension;
use std::sync::Arc;

pub async fn delete_user(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
    Json(delete_info): Json<DeleteRequest>,
) -> StatusCode {
    if local_user.user_role != Admin && local_user.name != delete_info.name {
        StatusCode::UNAUTHORIZED
    } else {
        match app_state.userdata.delete_user(&delete_info.name) {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
