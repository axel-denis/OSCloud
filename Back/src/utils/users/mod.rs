use std::{fs, path::PathBuf};

use crate::database::{model::User, UserData};

#[must_use]
pub struct VerifiedUserPath {
    user: User,
    path: PathBuf,
}

impl VerifiedUserPath {
    pub fn user(&self) -> &User {
        &self.user
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
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
                    user,
                    path: canonical,
                });
            };
        }
    }
    None
}
