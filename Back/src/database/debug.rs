use crate::database::UserData;

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
    pub fn users_mount_points_pretty_format(&self, usernames: Vec<&str>) -> Option<String> {
        use tabled::settings::Style;
        use tabled::Table;

        #[cfg_attr(feature = "cli", derive(tabled::Tabled))]
        struct Element {
            user: String,
            path: String,
        }
        let mut paths: Vec<Element> = Vec::new();
        for item in usernames.iter().skip(1) {
            let user = self.get_user_by_name(item)?;
            let mut paths_to_add: Vec<Element> = self
                .get_user_mounts_points(&user)?
                .iter()
                .map(|pth| Element {
                    user: item.to_string(),
                    path: pth.to_string(),
                })
                .collect();
            paths.append(&mut paths_to_add);
        }
        let mut table = Table::new(paths);

        table.with(Style::rounded());
        Some(table.to_string())
    }
}
