use crate::database::UserData;
use tabled::Table;
use tabled::settings::Style;

impl UserData {
    pub fn pretty_print(&self) {
        let mut table = Table::new(&self.users());

        table.with(Style::rounded());
        println!("{}", table);
    }
}
