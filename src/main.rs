mod users;

use users::get_users;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let token = std::env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set.");
    let users = get_users();

    println!("{:?} {:?}", users, token);
}
