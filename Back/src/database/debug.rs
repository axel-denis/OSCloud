use crate::database::UserData;
use super::model::User;

impl UserData {
    #[cfg(feature = "cli")]
    pub fn users_pretty_format(&self) -> crate::database::Result<String> {
        use tabled::settings::Style;
        use tabled::Table;

        let users = self.get_users()?;
        if users.is_empty() {
            return Ok("No users registered, the database is empty".to_string());
        }
        let mut table = Table::new(users);

        table.with(Style::rounded());
        Ok(table.to_string())
    }

    #[cfg(feature = "cli")]
    pub fn users_mount_points_pretty_format(&self, user_name: &str)
    -> Option<String> {

        use tabled::settings::Style;
        use tabled::Table;


        let user = self.get_user_by_name(user_name)?;
        let paths = self.get_user_mounts_points(&user)?;
        let mut table = Table::new(paths);

        table.with(Style::rounded());
        Some(table.to_string())
    }
}
