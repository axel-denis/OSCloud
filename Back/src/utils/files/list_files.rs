use core::fmt;
use std::{ffi::OsString, fs::read_dir};

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

pub struct FileInfo {
    pub name: OsString,
    pub file_type: FileType,
}

pub fn list_files(path: String) -> Result<Vec<FileInfo>, std::io::Error> {
    let paths = read_dir(path)?;

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
