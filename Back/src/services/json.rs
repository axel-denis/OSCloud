use crate::database::{
    model::{Role, User},
    UserData,
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

pub async fn save_to_json(db: web::Data<UserData>, req: HttpRequest) -> impl Responder {
    if let Some(local_user) = req.extensions().get::<User>() {
        if local_user.user_role == Role::Admin {
            return match db.save_default() {
                Ok(_) => HttpResponse::Ok().body("done"),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            };
        }
    }
    HttpResponse::Unauthorized().finish()
}

pub async fn import_from_json(db: web::Data<UserData>, req: HttpRequest) -> impl Responder {
    if let Some(local_user) = req.extensions().get::<User>() {
        if local_user.user_role == Role::Admin {
            return match db.import_default() {
                Ok(_) => HttpResponse::Ok().body("done"),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            };
        }
    }
    HttpResponse::Unauthorized().finish()
}
