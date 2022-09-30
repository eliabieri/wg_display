use common::models::Configuration;

pub struct Persistence {
    db: sled::Db,
}

impl Persistence {
    pub fn new() -> Self {
        Self {
            db: sled::open("wg_display.db").unwrap(),
        }
    }

    pub fn save_config(&self, config: Configuration) {
        self.db
            .insert("config_text", config.example_value.as_bytes())
            .unwrap();
    }

    pub fn get_config(&self) -> Option<Configuration> {
        let config_text = self
            .db
            .get("config_text")
            .unwrap()
            .map(|v| String::from_utf8(v.to_vec()).unwrap_or_else(|_| String::from("")))
            .unwrap();
        Some(Configuration {
            example_value: config_text,
        })
    }
}
