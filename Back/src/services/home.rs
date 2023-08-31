use crate::database::model::User;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};

pub async fn home(req: HttpRequest) -> impl Responder {
    if let Some(local_user) = req.extensions().get::<User>() {
        HttpResponse::Ok().json(local_user)
    } else {
        HttpResponse::NotFound().finish()
    }
}
