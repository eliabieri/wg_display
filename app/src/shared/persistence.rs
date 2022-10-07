use common::models::Configuration;
use rocket::serde::json::serde_json;

static DB_NAME: &str = "wg_display.db";
lazy_static! {
    static ref DB: sled::Db = sled::open(DB_NAME).expect("Could not open DB");
}

pub struct Persistence {}

impl Persistence {
    pub fn save_config(config: Configuration) {
        let serialized = serde_json::to_string(&config).expect("Could not serialize config");
        DB.insert("configuration", serialized.as_bytes())
            .expect("Could not save configuration");
    }

    pub fn get_config() -> Option<Configuration> {
        let configuration_text = DB
            .get("configuration")
            .expect("No configuration in DB")
            .map(|v| String::from_utf8(v.to_vec()).unwrap())
            .expect("Could not deserialize configuration");
        Some(serde_json::from_str(&configuration_text).expect("Could not serialize configuration"))
    }
}
