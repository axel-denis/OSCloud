// mod auth_middleware;
// #[cfg(feature = "cli")]
// mod cli;
// mod database;
// mod jwt_manager;
// mod network;
mod services;

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

use std::sync::Arc;
mod jwt_manager;

use axum::{routing::get, routing::post, Router};
use services::login::login ;
use tokio;

mod cli;
mod database;
use database::UserData;
use dotenv::dotenv;

pub struct AppState {
    userdata: UserData,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");

    let shared_state = Arc::new(AppState {
        userdata: UserData::new(),
    });
    // let userdata = UserData::new();

    #[cfg(feature = "cli")]
    cli::start_cli(&shared_state.userdata);

    let app = Router::new()
        .route("/login", post(login))
        .route(
            "/user",
            post(|| async { "Hello, World!" })
                .delete(|| async { "Hello, World!" })
                .get(|| async { "Hello, World!" }),
        )
        .route("/save", post(|| async { "Hello, World!" }))
        .route("/import", post(|| async { "Hello, World!" }))
        .route("/file", post(|| async { "Hello, World!" }))
        .route("/home", get(|| async { "Hello, World!" }))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888")
        .await
        .expect("failed to launch server");
    axum::serve(listener, app)
        .await
        .expect("failed to launch server");
}
