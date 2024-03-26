use core::fmt;
use std::{ffi::OsString, fs::read_dir};

use serde::Serialize;

use crate::utils::users::VerifiedUserPath;

#[derive(Serialize)]
pub enum FileType {
    FILE,
    FOLDER,
    OTHER,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::FILE => write!(f, "File"),
            FileType::FOLDER => write!(f, "Folder"),
            FileType::OTHER => write!(f, "Other"),
        }
    }
}

#[derive(Serialize)]
pub struct FileInfo {
    pub name: OsString,
    pub file_type: FileType,
}

// Please generate user path only with
pub fn list_files(user_path: &VerifiedUserPath) -> Result<Vec<FileInfo>, std::io::Error> {
    let paths = read_dir(user_path.path())?;

    Ok(paths
        .flatten()
        .map(|pth| FileInfo {
            name: pth.file_name(),
            file_type: if pth.path().is_file() {
                FileType::FILE
            } else if pth.path().is_dir() {
                FileType::FOLDER
            } else {
                FileType::OTHER
            },
        })
        .collect())
}
