use core::fmt;
use std::{ffi::OsString, fs::read_dir};

use serde::Serialize;

use crate::utils::users::VerifiedUserPath;

#[derive(Serialize)]
pub enum FileType {
    File,
    Folder,
    Other,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::File => write!(f, "File"),
            FileType::Folder => write!(f, "Folder"),
            FileType::Other => write!(f, "Other"),
        }
    }
}

#[derive(Serialize)]
pub struct FileInfo {
    pub name: OsString,
    pub file_type: FileType,
}

// Please generate user path only with verifiy_user_path
pub fn list_files(user_path: &VerifiedUserPath) -> Result<Vec<FileInfo>, std::io::Error> {
    let paths = read_dir(user_path.path())?;

    Ok(paths
        .flatten()
        .map(|pth| FileInfo {
            name: pth.file_name(),
            file_type: if pth.path().is_file() {
                FileType::File
            } else if pth.path().is_dir() {
                FileType::Folder
            } else {
                FileType::Other
            },
        })
        .collect())
}
