mod commands;

use std::io::BufRead;
use std::thread;
use colored::Colorize;

use crate::cli::commands::CommandsMap;
use crate::database::UserDatabase;

pub fn start_cli(db: &UserDatabase) {
    let db = db.clone();
    thread::spawn(move || {
        println!("Write {} to get the different commands", "help".bold());
        let map: CommandsMap = commands::create_commands_map();
        loop {
            for maybe_line in std::io::stdin().lock().lines() {
                if let Ok(line) = maybe_line {
                    match map.get(&line) {
                        Some(commands) => commands(&db),
                        None => println!("Unreconnized cmd!"),
                    }
                }
            }
        }
    });
}
