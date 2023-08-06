mod commands;

use std::io::{BufRead, Write};
use std::thread;
use colored::Colorize;

use crate::cli::commands::CommandsMap;
use crate::database::UserDatabase;

pub fn start_cli(db: &UserDatabase) {
    let db = db.clone();
    thread::spawn(move || {
        println!("Write {} to get the different commands", "help".bold());
        print!("{} #$ ", "OsCloud".cyan());
        std::io::stdout().flush().unwrap();

        let map: CommandsMap = commands::create_commands_map();
        loop {
            for maybe_line in std::io::stdin().lock().lines() {
                if let Ok(line) = maybe_line {
                    match map.get(&line) {
                        Some(commands) => commands(Vec::new(), &db),
                        None => println!("Unreconnized cmd!"),
                    }
                }
                print!("{} #$ ", "OsCloud".cyan());
                std::io::stdout().flush().unwrap();
            }
        }
    });
}
