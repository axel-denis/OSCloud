use crate::cli::commands::CmdStatus;
use crate::cli::formating::{err_str, ok_str};
use crate::database::{model::Role, UserData};
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) fn debug_users(_: Vec<&str>, db: &UserData) -> CmdStatus {
    match db.users_pretty_format() {
        Ok(str) => println!("{str}"),
        Err(err) => println!("{}", err_str(err)),
    }
    CmdStatus::Ok
}

pub(crate) fn create_user(args: Vec<&str>, db: &UserData) -> CmdStatus {
    if args.len() != 4 {
        println!(
            "{} create_user <username> <password> <(Admin|User)>{} help 'create_user'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        return CmdStatus::Ok;
    }

    let role = match Role::from_str(args[3]) {
        Ok(role_found) => role_found,
        Err(_) => {
            println!(
                "{} Admin {} User{} help 'create_user'",
                err_str("Invalid user type given, should be"),
                err_str("or"),
                err_str(", for more informations try")
            );
            return CmdStatus::Ok;
        }
    };

    match db.create_user(args[1], args[2], role) {
        Ok(user) => println!("User {} created!", ok_str(user.name)),
        Err(err) => println!("{}", err_str(err)),
    }
    CmdStatus::Ok
}

pub(crate) fn delete_user(args: Vec<&str>, db: &UserData) -> CmdStatus {
    if args.len() != 2 {
        println!(
            "{} delete_user <username>{} help 'delete_user'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        return CmdStatus::Ok;
    }

    match db.delete_user(args[1]) {
        Ok(_) => println!("User {} deleted!", ok_str(args[1])),
        Err(err) => println!("{}", err_str(err)),
    }
    CmdStatus::Ok
}

pub(crate) fn debug_user_mounts_points(args: Vec<&str>, db: &UserData) -> CmdStatus {
    if args.len() <= 1 {
        println!(
            "{} user_mounts_points [<username>]{} help 'user_mounts_points'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        return CmdStatus::Ok;
    }
        match db.users_mount_points_pretty_format(args[1..].to_vec()) {
            Some(str) => println!("{str}"),
            None => println!("{}", err_str(format!("User {} not found", args[0]))), // FIXME
        }
    CmdStatus::Ok
}

pub(crate) fn add_user_mount_point(args: Vec<&str>, db: &UserData) -> CmdStatus {
    if args.len() != 3 {
        println!(
            "{} add_user_mount_point <username>, <path>{} help 'add_user_mount_point'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        return CmdStatus::Ok;
    }

    let user = match db.get_user_by_name(args[1]) {
        Some(user) => user,
        None => {
            println!("User not found.");
            return CmdStatus::Ok
        },
    };
    match db.add_user_mount_point(&user, args[2].to_string()) {
        Ok(_) => println!("Mount point added!"),
        Err(err) => println!("{}", err_str(err)),
    };
    CmdStatus::Ok
}

pub(crate) fn save(args: Vec<&str>, db: &UserData) -> CmdStatus {
    if !(1..3).contains(&args.len()) {
        println!(
            "{} save [path]{} help 'save'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        return CmdStatus::Ok;
    }

    match args.get(1) {
        Some(path_str) => {
            let mut path = PathBuf::from(path_str);

            if path.is_dir() {
                path.push("users.json")
            }

            if let Err(err) = db.save_to_json(&path) {
                println!("{}", err_str(err))
            } else {
                println!("Database {}!", ok_str("saved"))
            }
        }
        None => {
            if let Err(err) = db.save_default() {
                println!("{}", err_str(err))
            } else {
                println!("Database {}!", ok_str("saved"))
            }
        }
    }
    CmdStatus::Ok
}

pub(crate) fn import(args: Vec<&str>, db: &UserData) -> CmdStatus {
    if !(1..3).contains(&args.len()) {
        println!(
            "{} import [path]{} help 'import'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        return CmdStatus::Ok;
    }
    match args.get(1) {
        Some(path_str) => {
            let mut path = PathBuf::from(path_str);

            if path.is_dir() {
                path.push("users.json")
            }

            if let Err(err) = db.import_from_json(&path) {
                println!("{}", err_str(err))
            } else {
                println!("Database {}!", ok_str("imported"))
            }
        }
        None => {
            if let Err(err) = db.import_default() {
                println!("{}", err_str(err))
            } else {
                println!("Database {}!", ok_str("imported"))
            }
        }
    }
    CmdStatus::Ok
}
