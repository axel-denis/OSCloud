use std::fs;
use std::process::exit;

use bcrypt::{hash, DEFAULT_COST};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::database::model::{
    FileShare, FileShareUser, NewFileShare, NewFileShareUser, NewUserMountPoint, ShareType,
    UserMountPoint,
};
use crate::database::model::{NewUser, Role, User};
use crate::database::schema::files_shares::dsl::files_shares;
use crate::database::schema::files_shares::link;
use crate::database::schema::files_shares_users::dsl::files_shares_users;
use crate::database::schema::files_shares_users::file_share_id;
use crate::database::schema::files_shares_users::shared_to;
use crate::database::schema::users::dsl::users;
use crate::database::schema::users::name;
use crate::database::Result;
use crate::database::{PostgresPool, UserData};
use crate::utils::files::file_info::check_path_is_folder;
use crate::utils::users::VerifiedUserPath;

impl UserData {
    // NOTE - nothing prevent from creating multiples shares per file, but currently
    // this shouldn't be done as everything can be stored in one + not all get functions
    // implement getting many elements and will often returns the first found
    // multiples shares will be useful when we will implement share rules (passwords, expirations...)
    // Currently, fronts only need to implement taking the first element of the given list when
    // getting files shares from the back
    pub fn add_file_share(
        &self,
        user: &User,
        path: &VerifiedUserPath,
        share_type: ShareType,
    ) -> Result<FileShare> {
        let mut pool = self.pool.get()?;
        Ok(
            diesel::insert_into(crate::database::schema::files_shares::dsl::files_shares)
                .values(&NewFileShare {
                    owner_user_id: user.id,
                    path: path.path().to_string_lossy().to_string(),
                    share_type,
                    link: "abcdef".to_string(), // TODO random string
                })
                .get_result::<FileShare>(&mut pool)?,
        )
    }

    pub fn get_share_from_code(&self, code: &String) -> Option<FileShare> {
        files_shares
            .filter(link.eq(code))
            .first::<FileShare>(&mut self.pool.get().ok()?)
            .ok()
    }

    pub fn get_share_from_file_path(&self, path: &VerifiedUserPath) -> Option<Vec<FileShare>> {
        files_shares
            .filter(crate::database::schema::files_shares::path.eq(path.path().to_string_lossy().to_string()))
            .get_results::<FileShare>(&mut self.pool.get().ok()?)
            .ok()
    }

    pub fn get_share_from_id(&self, id: i32) -> Option<FileShare> {
        files_shares
            .find(id)
            .first::<FileShare>(&mut self.pool.get().ok()?)
            .ok()
    }

    // get all files shared toward an user
    pub fn get_shared_to_user(&self, user: &User) -> Option<Vec<FileShare>> {
        let output: Vec<FileShare> = files_shares_users
            .filter(shared_to.eq(user.id))
            .get_results::<FileShareUser>(&mut self.pool.get().ok()?)
            .ok()?
            .iter()
            .filter_map(|share| self.get_share_from_id(share.file_share_id))
            .collect();
        // NOTE - the filter discards all not valid elements (their shouldn't be)
        // but still they are not "properly" handled
        // -> does not return internal server error in case of database failure
        Some(output)
    }

    // get all users IDs that have acces to the file via sharing
    pub fn get_file_users_shared_to(&self, path: &VerifiedUserPath) -> Option<Vec<i32>> {
        let shares = self.get_share_from_file_path(path)?;
        let mut output: Vec<i32> = Vec::new();
        for share in shares {
            output.append(
                &mut files_shares_users
                    .filter(file_share_id.eq(share.id))
                    .get_results::<FileShareUser>(&mut self.pool.get().ok()?)
                    .ok()?
                    .iter()
                    .map(|user_share| user_share.shared_to)
                    .collect(),
            );
        }
        // NOTE - the filter discards all not valid elements (their shouldn't be)
        // but still they are not "properly" handled
        // -> does not return internal server error in case of database failure
        Some(output)
    }

    pub fn add_file_share_user(
        &self,
        share: &FileShare,
        user_to_share_to: &User,
    ) -> Result<FileShareUser> {
        let mut pool = self.pool.get()?;
        Ok(diesel::insert_into(
            crate::database::schema::files_shares_users::dsl::files_shares_users,
        )
        .values(&NewFileShareUser {
            file_share_id: share.id,
            shared_to: user_to_share_to.id,
        })
        .get_result::<FileShareUser>(&mut pool)?)
    }
}
