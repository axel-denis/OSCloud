mod commands;
mod help;
mod users;
mod formating;

use std::io::{BufRead, Write};
use std::thread;
use colored::Colorize;

use crate::cli::commands::CommandsMap;
use crate::database::UserData;

pub fn execute_command(db: &UserData, line: String, map: &CommandsMap) {
    let parts: Vec<&str> = line.split(" ").collect();
    match map.get(parts[0]) {
        Some(commands) => commands(parts, &db),
        None => println!("Unrecognized cmd!"),
    }
}

pub fn start_cli(db: &UserData) {
    let db = db.clone();
    thread::spawn(move || {
        println!("Write {} to get the different commands", "help".bold());
        print!("{} #$ ", "OsCloud".cyan());
        std::io::stdout().flush().unwrap();

        let map: CommandsMap = commands::create_commands_map();
        loop {
            for maybe_line in std::io::stdin().lock().lines() {
                if let Ok(line) = maybe_line {
                    execute_command(&db, line, &map);
                }
                print!("{} #$ ", "OsCloud".cyan());
                std::io::stdout().flush().unwrap();
            }
        }
    });
}
