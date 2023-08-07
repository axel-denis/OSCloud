use std::collections::HashMap;
use colored::Colorize;
use crate::database::UserDatabase;

pub type CommandsFn = fn(Vec<&str>, &UserDatabase) -> ();

pub type CommandsMap = HashMap<String, CommandsFn>;


pub(crate) fn create_commands_map() -> CommandsMap {
    let mut map = CommandsMap::new();

    map.insert("h".to_owned(), crate::cli::help::help);
    map.insert("help".to_owned(), crate::cli::help::help);
    map.insert("u".to_owned(), debug_users);
    map.insert("users".to_owned(), debug_users);
    map.insert("e".to_owned(), exit);
    map.insert("exit".to_owned(), exit);
    map.insert("q".to_owned(), exit);
    map.insert("quit".to_owned(), exit);
    map.insert("c".to_owned(), clear);
    map.insert("clear".to_owned(), clear);
    map
}

pub(crate) fn clear(_: Vec<&str>, _: &UserDatabase) {
    if let Err(err) = clearscreen::clear() {
        println!("{}", err.to_string().red().bold());
    }
}

fn debug_users(_: Vec<&str>, db: &UserDatabase) {
    db.pretty_print()
}

pub(crate) fn exit(args: Vec<&str>, db: &UserDatabase) {
    println!("Exiting...");
    if let Some(&flag) = args.get(1) {
        if flag == "--no-backup" || flag == "-n" {
            if let Err(err) = db.save_to_json() {
                println!("{}", err.to_string().red().bold());
            }
        }
    }
    std::process::exit(0);
}
