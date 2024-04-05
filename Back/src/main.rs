#![warn(unused_extern_crates)]

mod auth_middleware;
mod services;
mod utils;
use std::sync::Arc;
mod jwt_manager;

use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use axum::middleware;
use axum::{routing::get, routing::post, Router};
use services::file_upload::{download, upload};
use services::files_management::{delete_file, move_file, rename_file};
use services::files_share::{file_shared_info, list_shared_to_me, share_file};
use services::home::home;
use services::json::import_from_json;
use services::json::save_to_json;
use services::login::login;
use services::user_info::{get_users_enableness, user_info};
use services::user_management::{add_user, delete_user, enable_user};

mod cli;
mod database;
use database::UserData;
use dotenv::dotenv;

use tower_http::cors::{Any, CorsLayer};

use crate::services::list_files::list_files;

#[derive(Clone)]
pub struct AppState {
    userdata: UserData,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    std::env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set.");

    let shared_state = Arc::new(AppState {
        userdata: UserData::new(),
    });

    #[cfg(feature = "cli")]
    cli::start_cli(&shared_state.userdata);

    // NOTE - patched to allow dev. Need to be looked into further
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let all_router = Router::new()
        .route("/login", post(login))
        .with_state(shared_state.clone());

    let registered_router = Router::new()
        .route(
            "/user",
            post(|| async { "Hello, World!" })
                .delete(delete_user)
                .get(user_info),
        )
        .route("/save", post(save_to_json))
        .route("/import", post(import_from_json))
        .route("/file", post(|| async { "Hello, World!" }))
        .route("/list_files/:dir", get(list_files))
        .route("/home", get(home))
        .route("/upload", post(upload))
        .route("/files_management/rename", post(rename_file))
        .route("/files_management/move", post(move_file))
        .route("/files_management/delete", post(delete_file))
        .route("/share_file/", post(share_file))
        .route("/share_file/", get(file_shared_info))
        .route("/shared_to_me/", get(list_shared_to_me))
        .nest_service(
            "/download/:file",
            get(download).with_state(shared_state.clone()),
        )
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth_middleware::auth_middleware,
        ))
        .with_state(shared_state.clone());
    let admin_router = Router::new()
        .route("/add_user", post(add_user))
        .route("/enable_user", post(enable_user))
        .route("/enable_user", get(get_users_enableness))
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth_middleware::admin_middleware,
        ))
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth_middleware::auth_middleware,
        ))
        .with_state(shared_state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888")
        .await
        .expect("failed to launch server");
    axum::serve(
        listener,
        all_router
            .merge(registered_router)
            .merge(admin_router)
            .layer(cors),
    )
    .await
    .expect("failed to launch server");
}
