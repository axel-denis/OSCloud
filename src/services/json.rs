use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use crate::database::{model::{User, Role}, UserDatabase};

pub async fn save_to_json(db: web::Data<UserDatabase>, req: HttpRequest) -> impl Responder {
    if let Some(local_user) = req.extensions().get::<User>() {
        if local_user.user_role == Role::Admin {
            return match db.save_to_json() {
                Ok(_) => HttpResponse::Ok().body("done"),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            };
        }
    }
    HttpResponse::Unauthorized().finish()
}

pub async fn import_from_json(db: web::Data<UserDatabase>, req: HttpRequest) -> impl Responder {
    if let Some(local_user) = req.extensions().get::<User>() {
        if local_user.user_role == Role::Admin {
            return match db.import_from_json() {
                Ok(_) => HttpResponse::Ok().body("done"),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            };
        }
    }
    HttpResponse::Unauthorized().finish()
}
