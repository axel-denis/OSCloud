use crate::database::UserData;
use crate::database::model::{Tag, NewTag};

use crate::database::schema::tags::dsl::tags;
use crate::database::Result;
use crate::database::schema::tags::name;

use diesel::prelude::*;

impl UserData {
    pub fn get_tags(&self) -> Result<Vec<Tag>> {
        Ok(tags.load::<Tag>(&mut self.pool.get()?)?)
    }

    pub fn create_tag(&self, tag_name: &str) -> Result<Tag> {
        let mut pool = self.pool.get()?;

        if !tags
            .filter(name.eq(tag_name.clone()))
            .get_results::<Tag>(&mut pool)?
            .is_empty()
        {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Tag already exist",
            )));
        }
        diesel::insert_into(tags)
            .values(&NewTag{
                name: tag_name.to_string(),
            })
            .execute(&mut pool)?;
        Ok(self.get_tag_by_name(tag_name).ok_or("Not Found")?)
    }

    pub fn get_tag_by_name(&self, tag_name: &str) -> Option<Tag> {
        tags
            .filter(name.eq(tag_name))
            .first::<Tag>(&mut self.pool.get().ok()?)
            .ok()
    }
}
