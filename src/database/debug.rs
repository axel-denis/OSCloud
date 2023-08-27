use crate::database::UserData;
use tabled::settings::Style;
use tabled::Table;
use crate::database::Result;

impl UserData {
    pub fn pretty_format(&self) -> Result<String> {
        let users = self.get_users()?;
        if users.is_empty() {
            return Ok("No users registered, the database is empty".to_string());
        }
        let mut table = Table::new(users);

        table.with(Style::rounded());
        Ok(table.to_string())
    }
}
