use std::collections::HashMap;
use colored::Colorize;
use crate::database::UserData;

pub type CommandsFn = fn(Vec<&str>, &UserData) -> ();

pub type CommandsMap = HashMap<String, CommandsFn>;


pub(crate) fn create_commands_map() -> CommandsMap {
    let mut map = CommandsMap::new();

    map.insert("h".to_owned(), crate::cli::help::help);
    map.insert("help".to_owned(), crate::cli::help::help);
    map.insert("u".to_owned(), crate::cli::users::debug_users);
    map.insert("users".to_owned(), crate::cli::users::debug_users);
    map.insert("e".to_owned(), exit);
    map.insert("exit".to_owned(), exit);
    map.insert("q".to_owned(), exit);
    map.insert("quit".to_owned(), exit);
    map.insert("c".to_owned(), clear);
    map.insert("clear".to_owned(), clear);
    map.insert("s".to_owned(), crate::cli::users::save);
    map.insert("save".to_owned(), crate::cli::users::save);
    map.insert("i".to_owned(), crate::cli::users::import);
    map.insert("import".to_owned(), crate::cli::users::import);
    map.insert("cu".to_owned(), crate::cli::users::create_user);
    map.insert("create_user".to_owned(), crate::cli::users::create_user);
    map.insert("du".to_owned(), crate::cli::users::delete_user);
    map.insert("delete_user".to_owned(), crate::cli::users::delete_user);
    map
}

pub(crate) fn clear(_: Vec<&str>, _: &UserData) {
    if let Err(err) = clearscreen::clear() {
        println!("{}", err.to_string().red().bold());
    }
}

pub(crate) fn exit(args: Vec<&str>, db: &UserData) {
    if let Some(&flag) = args.get(1) {
        if flag == "--no-backup" || flag == "-n" {
            println!("Exiting...");
            std::process::exit(0);
        }
    }
    crate::cli::users::save(vec!["save"], db);
    println!("Exiting...");
    std::process::exit(0);
}
