use std::{io::Write, sync::{Arc, Mutex}};

use crate::database::{
    model::User,
    UserData,
};
use actix_multipart::{Multipart, Field};
use actix_web::{web::{self, Buf}, HttpMessage, HttpRequest, HttpResponse, Responder};
use futures_util::{TryStreamExt, StreamExt};

pub async fn get_path<'a>(mut field: Field) -> Result<String, Box<dyn std::error::Error>> {
    let mut path = String::new();

    while let Some(chunk) = field.next().await {
        // filesystem operations are blocking, we have to use threadpool
        path.push_str(std::str::from_utf8(chunk?.chunk())?);
    }
    Ok(path)
}

pub async fn get_file(data: &web::Data<UserData>, mut field: Field, path: &String) -> Result<(), Box<dyn std::error::Error>> {
    //let file_type = field.content_type().ok_or("No file name")?;
    let file_name = field
        .content_disposition()
        .get_filename()
        .ok_or("File name not accessible")?
        .to_owned();
    let path = format!("{}/data/{}/", data.dirs.config_dir().to_string_lossy(), path);

    let file = web::block(move || {
        std::fs::create_dir_all(&path)?;
        std::fs::File::create(format!("{path}{file_name}"))
    })
        .await??;

    let f = Arc::new(Mutex::new(file));
    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        let file = Arc::clone(&f);
        // filesystem operations are blocking, we have to use threadpool
        web::block(move || file.lock().unwrap().write_all(&data))
            .await??;
    }
    Ok(())
}

pub async fn upload(data: web::Data<UserData>, mut payload: Multipart, req: HttpRequest) -> impl Responder {
    if let Some(_) = req.extensions().get::<User>() {
        let mut last_path : Option<String> = None;
        while let Ok(Some(field)) = payload.try_next().await {
            if field.name() == "path" {
                match get_path(field).await {
                    Ok(str) => last_path = Some(str),
                    Err(_) => return HttpResponse::InternalServerError().finish(),
                };
            } else if field.name() == "file" {
                if let Some(path) = &last_path {
                    if get_file(&data, field, path).await.is_err() {
                        return HttpResponse::InternalServerError().finish()
                    };
                } else {
                    return HttpResponse::InternalServerError().finish()
                }
            }
        }
        return HttpResponse::Created().finish();
    }
    HttpResponse::Unauthorized().finish()
}
