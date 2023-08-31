use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{
    database::UserData,
    services::{login, register, delete, user_info, json, home, upload},
};

#[actix_web::main]
pub async fn launch_actix(userdata: UserData) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(userdata.clone()))
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(crate::auth_middleware::Auth)
            .route("/login", web::post().to(login::login))
            .route("/user", web::post().to(register::register))
            .route("/user",web::delete().to(delete::delete_user))
            .route("/user", web::get().to(user_info::user_info))
            .route("/save", web::post().to(json::save_to_json))
            .route("/import", web::post().to(json::import_from_json))
            .route("/file", web::post().to(upload::get_file))
            .route("/home", web::get().to(home::home))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
