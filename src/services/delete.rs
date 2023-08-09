use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use crate::database::model::User;
use crate::database::model::Role::Admin;
use crate::database::UserData;

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    name: String,
}

pub async fn delete_user(db: web::Data<UserData>, req: HttpRequest, delete_info: web::Json<DeleteRequest>) -> impl Responder {
    if let Some(local_user) = req.extensions().get::<User>() {
        if local_user.user_role != Admin && local_user.name != delete_info.name {
            return HttpResponse::Unauthorized().body("Bad permission");
        }

        match db.delete_user(&delete_info.name) {
            Ok(_) => {HttpResponse::Ok().body("User deleted")}
            Err(_) => {HttpResponse::Unauthorized().finish()}
        }
    }
    HttpResponse::Unauthorized().finish()
}