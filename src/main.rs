mod users;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use users::get_users;
use dotenv::dotenv;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main_actix() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn main() {
    dotenv().ok();
    let token = std::env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set.");
    let users = get_users();

    println!("{:?} {:?}", users, token);
    main_actix().unwrap();
}
