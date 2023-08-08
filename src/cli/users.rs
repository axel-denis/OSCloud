use crate::database::{UserData, model::Role};
use std::str::FromStr;
use crate::cli::formating::{err_str, ok_str};
use std::path::PathBuf;

pub(crate) fn debug_users(_: Vec<&str>, db: &UserData) {
    db.pretty_print()
}

pub(crate) fn create_user(args: Vec<&str>, db: &UserData) {
    if args.len() != 4 {
        println!("{} create_user <name> <password> <(Admin|User)>{} help 'create_user'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        return
    }

    let role = match Role::from_str(args[3]) {
        Ok(role_found) => role_found,
        Err(_) => {
            println!("{} Admin {} User{} help 'create_user'",
                err_str("Invalid user type given, should be"),
                err_str("or"),
                err_str(", for more informations try"));
            return
        },
    };

    match db.create_user(args[1], args[2], role) {
        Ok(user) => println!("User {} created!", ok_str(user.name)),
        Err(err) => println!("{}", err_str(&err.to_string())),
    }
}

pub(crate) fn save(args: Vec<&str>, db: &UserData) {
    if !(1..3).contains(&args.len()) {
        println!("{} save [path]{} help 'save'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        return
    }
    match args.get(1) {
        Some(path_str) => {
            let mut path = PathBuf::from(path_str);

            if path.is_dir() {
                path.push("users.json")
            }

            if let Err(err) = db.save_to_json(&path) {
                println!("{}", err_str(err));
            } else {
                println!("Database {}!", ok_str("saved"))
            }
        },
        None => {
            if let Err(err) = db.save_default() {
                println!("{}", err_str(err));
            } else {
                println!("Database {}!", ok_str("saved"))
            }
        },
    }
}

pub(crate) fn import(args: Vec<&str>, db: &UserData) {
    if !(1..3).contains(&args.len()) {
        println!("{} import [path]{} help 'import'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        return
    }
    match args.get(1) {
        Some(path_str) => {
            let mut path = PathBuf::from(path_str);

            if path.is_dir() {
                path.push("users.json")
            }

            if let Err(err) = db.import_from_json(&path) {
                println!("{}", err_str(err));
            } else {
                println!("Database {}!", ok_str("imported"))
            }
        },
        None => {
            if let Err(err) = db.import_default() {
                println!("{}", err_str(err));
            } else {
                println!("Database {}!", ok_str("imported"))
            }
        },
    }
}
