use std::fs;

use diesel::prelude::*;

use crate::database::model::User;
use crate::database::model::{NewUserMountPoint, UserMountPoint};
use crate::database::schema::users::dsl::enabled;
use crate::database::Result;
use crate::database::UserData;
use crate::utils::files::clean_path::clean_path;
use crate::utils::files::file_info::check_path_is_folder;

impl UserData {
    // pub fn get_user_mount_point_by_id(&self, id: i32) -> Option<UserMountPoint> {
    //     crate::database::schema::user_mounts_points::dsl::user_mounts_points
    //         .find(id)
    //         .get_result::<UserMountPoint>(&mut self.pool.get().ok()?)
    //         .ok()
    // }

    pub fn add_user_mount_point(&self, user: &User, path: &String) -> Result<UserMountPoint> {
        let pathed_path = match clean_path(path).into_os_string().into_string() {
            Ok(path) => path,
            Err(_) => return Err("Mount point could not be resolved into a string".into()),
        };
        if !check_path_is_folder(path) {
            return Err("Mount point need to be a valid folder".into());
        }
        let mut pool = self.pool.get()?;
        Ok(diesel::insert_into(
            crate::database::schema::user_mounts_points::dsl::user_mounts_points,
        )
        .values(&NewUserMountPoint {
            user_id: user.id,
            path: pathed_path,
        })
        .get_result::<UserMountPoint>(&mut pool)?)
    }

    pub fn get_user_mounts_points(&self, user: &User) -> Option<Vec<String>> {
        let pool = &mut self.pool.get().ok()?;
        let test: Vec<String> = UserMountPoint::belonging_to(&user)
            .select(UserMountPoint::as_select())
            .load::<UserMountPoint>(pool)
            .ok()?
            .iter()
            .map(|ump| ump.path.clone())
            .collect();
        Some(test)
    }

    // Enables or disable the given user based on the `state` argument
    pub fn enable_user(&self, user: &User, state: bool) -> Result<bool> {
        let mut pool = self.pool.get()?;

        Ok(diesel::update(user)
            .set(enabled.eq(state))
            .get_result::<User>(&mut pool)?
            .enabled)
    }
}
