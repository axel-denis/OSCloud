use std::path::Path;

use colored::Colorize;

use walkdir::WalkDir;

use crate::cli::commands::CmdStatus;
use crate::cli::formating::alt_str;
use crate::database::UserData;

pub(crate) fn tree(_: Vec<&str>, db: &UserData) -> CmdStatus {
    let position = format!(
        "{}/data",
        db.dirs.config_dir().as_os_str().to_str().unwrap()
    );
    let data_folder = Path::new(&position);

    println!("Tree: {:?}", data_folder);

    let _: Option<()> = WalkDir::new(data_folder)
        .min_depth(1)
        .into_iter()
        .flatten()
        .map(|item| {
            let path = item.into_path();

            if path.is_file() {
                println!(
                    "{}/{}",
                    alt_str(path.parent()?.strip_prefix(data_folder).ok()?.display()),
                    path.file_name()?.to_str()?
                )
            } else {
                println!(
                    "{}",
                    path.strip_prefix(data_folder)
                        .ok()?
                        .display()
                        .to_string()
                        .magenta()
                )
            };
            Some(())
        })
        .collect();
    CmdStatus::Ok
}
