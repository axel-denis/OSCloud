use actix_web::{web, App, HttpServer};

#[actix_web::main]
pub async fn launch_actix() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(crate::services::login::login))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
