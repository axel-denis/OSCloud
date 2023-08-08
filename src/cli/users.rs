use crate::database::{UserDatabase, model::Role};
use std::str::FromStr;
use crate::cli::formating::{err_str, ok_str};

pub(crate) fn debug_users(_: Vec<&str>, db: &UserDatabase) {
    db.pretty_print()
}

pub(crate) fn create_user(args: Vec<&str>, db: &UserDatabase) {
    let user_name: &str = match args.get(1) {
        Some(argument) => *argument,
        None => {println!("{} create_user <name> <password> <(Admin|User)>{} help 'create_user'", err_str("Invalid arguments given, should be"), err_str(", for more informations try")); return},
    };

    let user_password: &str = match args.get(2) {
        Some(argument) => *argument,
        None => {println!("{} create_user <name> <password> <(Admin|User)>{} help 'create_user'", err_str("Invalid arguments given, should be"), err_str(", for more informations try")); return},
    };

    let role_str: &str = match args.get(3) {
        Some(argument) => *argument,
        None => {println!("{} create_user <name> <password> <(Admin|User)>{} help 'create_user'", err_str("Invalid arguments given, should be"), err_str(", for more informations try")); return},
    };

    let role = match Role::from_str(role_str) {
        Ok(role_found) => role_found,
        Err(_) => {println!("{} create_user <name> <password> <(Admin|User)>{} help 'create_user'", err_str("Invalid arguments given, should be"), err_str(", for more informations try")); return},
    };

    match db.create_user(user_name, user_password, role) {
        Ok(user) => println!("User {} created!", ok_str(user.name)),
        Err(err) => println!("{}", err_str(&err.to_string())),
    }
}

pub(crate) fn save(_: Vec<&str>, db: &UserDatabase) {
    if let Err(err) = db.save_to_json() {
        println!("{}", err_str(err));
    } else {
        println!("Database {}!", ok_str("saved"))
    }
}

pub(crate) fn import(_: Vec<&str>, db: &UserDatabase) {
    if let Err(err) = db.import_from_json() {
        println!("{}", err_str(err));
    } else {
        println!("Database {}!", ok_str("imported"))
    }
}
