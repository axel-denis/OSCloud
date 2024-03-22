use bcrypt::{hash, DEFAULT_COST};

use diesel::prelude::*;

use crate::database::model::{NewUser, Role, ShareableUser, User};
use crate::database::schema::users::dsl::users;
use crate::database::schema::users::name;
use crate::database::Result;
use crate::database::UserData;

impl UserData {
    /*
    pub fn users(&self) -> Vec<User> {
        self.get_users().unwrap()
    }
    */
    pub fn get_users(&self) -> Result<Vec<User>> {
        Ok(users.load::<User>(&mut self.pool.get()?)?)
    }
    /*
    pub fn delete_by_id(&self, id: i64) -> Result<()> {
        if users.find(id).count().first::<i64>(&mut self.pool.get()?)? <= 0 {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found")));
        }
        diesel::delete(users.find(id)).execute(&mut self.pool.get()?)?;
        Ok(())
    }*/

    pub fn delete_user(&self, user_name: &str) -> Result<()> {
        if users
            .filter(name.eq(user_name))
            .count()
            .first::<i64>(&mut self.pool.get()?)?
            <= 0
        {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "User not found",
            )));
        }
        diesel::delete(users.filter(name.eq(user_name))).execute(&mut self.pool.get()?)?;
        Ok(())
    }

    pub fn create_user(
        &self,
        user_name: &str,
        user_password: &str,
        role: Role,
        enable: bool,
    ) -> Result<User> {
        let mut pool = self.pool.get()?;
        if !users
            .filter(name.eq(user_name))
            .get_results::<User>(&mut pool)?
            .is_empty()
        {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "User already exist",
            )));
        }
        let hashed_password = hash(user_password, DEFAULT_COST)?;

        diesel::insert_into(users)
            .values(&NewUser {
                name: user_name.to_string(),
                password: hashed_password,
                user_role: role,
                enabled: enable,
            })
            .execute(&mut pool)?;
        Ok(self.get_user_by_name(user_name).ok_or("Not Found")?)
    }

    pub fn get_user_by_id(&self, user_id: i32) -> Option<User> {
        users
            .find(user_id)
            .get_result::<User>(&mut self.pool.get().ok()?)
            .ok()
    }

    pub fn get_user_by_name(&self, user_name: &str) -> Option<User> {
        users
            .filter(name.eq(user_name))
            .first::<User>(&mut self.pool.get().ok()?)
            .ok()
    }

    // get the user data without passwords
    pub fn get_users_public(&self) -> Result<Vec<ShareableUser>> {
        Ok(self
            .get_users()?
            .iter()
            .map(|user| ShareableUser {
                id: user.id,
                name: user.name.clone(),
                user_role: user.user_role.clone(),
                enabled: user.enabled,
            })
            .collect())
    }
}
