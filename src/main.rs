mod auth_middleware;
mod jwt_manager;
mod network;
mod services;
mod users;
mod register;
mod database;

use dotenv::dotenv;
fn main() {
    dotenv().ok();

    let a = register::register_new_user_in_database("NewUser", "Password", users::Type::Admin);
    if let Some(err) = a.err() {
        println!("{err}");
    }
    std::env::set_var("RUST_LOG", "actix_web=debug");
    network::launch_actix().expect("actix launch crashed");
}
