use crate::database::UserData;

impl UserData {
    #[cfg(feature = "cli")]
    pub fn pretty_format(&self) -> crate::database::Result<String> {
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
}
