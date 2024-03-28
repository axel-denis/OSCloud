use std::{fs, path::PathBuf};

use crate::database::{model::User, UserData};

pub struct VerifiedUserPath {
    _user: User,
    _path: PathBuf,
}

impl VerifiedUserPath {
    pub fn user(&self) -> &User {
        &self._user
    }
    pub fn path(&self) -> &PathBuf {
        &self._path
    }
}

pub fn verifiy_user_path(db: &UserData, path: &String, user: User) -> Option<VerifiedUserPath> {
    let mnts = db.get_user_mounts_points(&user)?;
    let canonical = match fs::canonicalize(path) {
        Ok(result) => result,
        Err(_) => return None,
    };
    for mnt in mnts {
        for ancestor in canonical.ancestors() {
            if ancestor == PathBuf::from(&mnt) {
                return Some(VerifiedUserPath {
                    _user: user,
                    _path: canonical,
                });
            };
        }
    }
    None
}
