use std::{io::Write, cell::{RefCell}, sync::{Arc, Mutex}};

use crate::database::{
    model::{Role, User},
    UserData,
};
use actix_multipart::Multipart;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use futures_util::{TryStreamExt, StreamExt};

pub async fn get_file(_: web::Data<UserData>, mut payload: Multipart, req: HttpRequest) -> impl Responder {
    if let Some(_) = req.extensions().get::<User>() {
        while let Ok(Some(mut field)) = payload.try_next().await {
            println!("{:?}", field);

            let filepath = format!("./new.png");

            // File::create is blocking operation, use threadpool
            let file = web::block(|| std::fs::File::create(filepath).unwrap())
                .await
                .unwrap();

            let f = Arc::new(Mutex::new(file));
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use threadpool
                let file = Arc::clone(&f);
                web::block(move || file.lock().unwrap().write_all(&data))
                    .await
                    .unwrap()
                    .unwrap();
            }
        }
    }
    HttpResponse::Unauthorized().finish()
}
