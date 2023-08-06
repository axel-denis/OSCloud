use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;

use crate::{database::UserDatabase, services::{login, register}};

#[actix_web::main]
pub async fn launch_actix(userbase: UserDatabase) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move  ||{
        App::new()
            .app_data(web::Data::new(userbase.clone()))
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(crate::auth_middleware::Auth)
            .route("/login", web::post().to(login::login))
            .route("/register", web::post().to(register::register))
            .route(
                "/userInfo",
                web::get().to(crate::services::user_info::user_info),
            )
            .route("/home", web::get().to(crate::services::home::home))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
