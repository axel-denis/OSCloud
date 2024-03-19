use crate::cli::commands::CmdStatus;
use crate::cli::formating::err_str;
use crate::database::UserData;
use crate::utils::files;

pub(crate) fn list_files(args: Vec<&str>, _: &UserData) -> CmdStatus {
    if args.len() != 2 {
        println!(
            "{} list_files <path>{} help 'list_files'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        CmdStatus::Ok
    } else {
        let files = match files::list_files::list_files(args[1].to_owned()) {
            Ok(files) => files,
            Err(e) => {
                println!("Error while listing files : {}", err_str(e));
                return CmdStatus::Ok;
            }
        };
        for file in files {
            println!(
                "{}:\t {}",
                file.file_type,
                file.name
                    .into_string()
                    .unwrap_or("### non unicode data ###".to_string())
            );
        }
        CmdStatus::Ok
    }
}
