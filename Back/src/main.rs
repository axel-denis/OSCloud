// mod auth_middleware;
// #[cfg(feature = "cli")]
// mod cli;
// mod database;
// mod jwt_manager;
// mod network;
// mod services;

// use database::UserData;
// use dotenv::dotenv;

// fn main() {
//     dotenv().ok();

//     std::env::set_var("RUST_LOG", "actix_web=debug");
//     let userdata = UserData::new();

//     #[cfg(feature = "cli")]
//     cli::start_cli(&userdata);
//     network::launch_actix(userdata).expect("actix launch crashed");
// }

use axum::{routing::delete, routing::get, routing::post, Router};
use tokio;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/login", post(|| async { "Hello, World!" }))
        .route("/user", post(|| async { "Hello, World!" }))
        .route("/user", delete(|| async { "Hello, World!" }))
        .route("/user", get(|| async { "Hello, World!" }))
        .route("/save", post(|| async { "Hello, World!" }))
        .route("/import", post(|| async { "Hello, World!" }))
        .route("/file", post(|| async { "Hello, World!" }))
        .route("/home", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to launch server");
    axum::serve(listener, app)
        .await
        .expect("failed to launch server");
}
