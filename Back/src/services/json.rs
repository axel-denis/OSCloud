// use crate::database::{
//     model::{Role, User},
//     UserData,
// };
// use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

// pub async fn save_to_json(db: web::Data<UserData>, req: HttpRequest) -> impl Responder {
//     if let Some(local_user) = req.extensions().get::<User>() {
//         if local_user.user_role == Role::Admin {
//             return match db.save_default() {
//                 Ok(_) => HttpResponse::Ok().body("done"),
//                 Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//             };
//         }
//     }
//     HttpResponse::Unauthorized().finish()
// }

// pub async fn import_from_json(db: web::Data<UserData>, req: HttpRequest) -> impl Responder {
//     if let Some(local_user) = req.extensions().get::<User>() {
//         if local_user.user_role == Role::Admin {
//             return match db.import_default() {
//                 Ok(_) => HttpResponse::Ok().body("done"),
//                 Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//             };
//         }
//     }
//     HttpResponse::Unauthorized().finish()
// }

use crate::database::model::{Role, User};
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Extension;
use std::sync::Arc;

pub async fn save_to_json(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
) -> StatusCode {
        if local_user.user_role == Role::Admin {
            return match app_state.userdata.save_default() {
                Ok(_) => StatusCode::OK,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
        }
    StatusCode::UNAUTHORIZED
}

pub async fn import_from_json(
    State(app_state): State<Arc<AppState>>,
    Extension(local_user): Extension<User>,
) -> StatusCode {
        if local_user.user_role == Role::Admin {
            return match app_state.userdata.import_default() {
                Ok(_) => StatusCode::OK,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
        }
    StatusCode::UNAUTHORIZED
}
