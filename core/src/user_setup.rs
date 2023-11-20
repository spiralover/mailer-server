pub struct UserSetup {
    pub menu_items: Vec<String>,
}

impl Default for UserSetup {
    fn default() -> Self {
        UserSetup {
            menu_items: vec![
                String::from("bb6eed4f-9fb4-49ec-bb46-e706dbcf6fc9"), // notifications
            ],
        }
    }
}

impl UserSetup {
    pub fn new() -> Self {
        Self::default()
    }
}
