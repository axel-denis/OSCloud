use crate::database::UserData;
use tabled::settings::Style;
use tabled::Table;
use crate::database::Result;

impl UserData {
    pub fn pretty_format(&self) -> Result<String> {
        let mut table = Table::new(self.get_users()?);

        table.with(Style::rounded());
        Ok(table.to_string())
    }
}
