use std::path::PathBuf;

use crate::database::{model::User, UserData};

use super::files::clean_path::clean_path;

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
    let clean_path = clean_path(path);
    for mnt in mnts {
        for ancestor in clean_path.ancestors() {
            if ancestor == PathBuf::from(&mnt) {
                return Some(VerifiedUserPath {
                    user,
                    path: clean_path,
                });
            };
        }
    }
    None
}
