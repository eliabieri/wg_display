pub struct Persistence {
    db: sled::Db,
}

impl Peristence {
    fn new() -> Self {
        sled::open("wg_display.db").unwrap();
    }

    fn save_config(&self, config: Config) {
        self.db
            .insert("config_text", config.text.as_bytes())
            .unwrap();
    }

    fn get_config(&self) -> Option<Config> {
        let config_text = self
            .db
            .get("config_text")
            .unwrap()
            .map(|v| String::from_utf8(v.to_vec()).unwrap_or_else(|_| String::from("")));
        Some(Config { text: config_text })
    }
}
