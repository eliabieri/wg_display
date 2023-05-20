//! Implementation of the system configuration persistence
use common::models::{SystemConfiguration, WidgetInstallationData};
use rocket::serde::json::serde_json;

use std::sync::atomic::{AtomicBool, Ordering};

static DB_NAME: &str = "wg_display.db";
lazy_static! {
    static ref DB: sled::Db = sled::open(DB_NAME).expect("Could not open DB");
}

static CONFIG_UPDATED: AtomicBool = AtomicBool::new(false);

pub struct Persistence {}

/// Persists the system configuration.
/// Uses the SLED embedded database.
impl Persistence {
    const DB_KEY: &str = "system_configuration";

    /// Save the system configuration
    /// # Arguments
    /// * `config` - The system configuration to save
    pub fn save_system_config(config: SystemConfiguration) {
        let serialized = serde_json::to_string(&config).expect("Could not serialize config");
        DB.insert(Persistence::DB_KEY, serialized.as_bytes())
            .expect("Could not save configuration");
        CONFIG_UPDATED.store(true, Ordering::Relaxed);
    }

    // Adds the default installation data for a widget
    // # Arguments
    // * `widget_name` - Name of the widget
    // * `description` - Description of the widget
    pub fn add_default_installation_data(widget_name: &str, description: &str) {
        let config = Persistence::get_system_config().unwrap();
        let mut widgets = config.widgets;
        widgets.push(WidgetInstallationData {
            name: widget_name.to_string(),
            description: description.to_owned(),
            json_config: "{}".to_string(),
        });
        let new_config = SystemConfiguration { widgets, ..config };
        Persistence::save_system_config(new_config);
    }

    /// Load the system configuration
    /// # Returns
    /// The system configuration
    pub fn get_system_config() -> Option<SystemConfiguration> {
        let config = DB
            .get(Persistence::DB_KEY)
            .expect("FATAL: Could not read DB");
        match config {
            Some(bytes) => {
                let config_str = String::from_utf8(bytes.to_vec())
                    .expect("Could not convert config bytes to string");
                Some(
                    serde_json::from_str(&config_str).expect("Could not deserialize configuration"),
                )
            }
            _ => {
                Persistence::create_default_system_config();
                Persistence::get_system_config()
            }
        }
    }

    /// Load json configuration for a specific widget
    /// # Returns
    /// The widget configuration
    pub fn get_widget_config(widget_name: &str) -> Option<String> {
        let config = Persistence::get_system_config();
        let Some(config) = config else {
            return None;
        };
        for widget in config.widgets {
            if widget.name == widget_name {
                return Some(widget.json_config);
            }
        }
        None
    }

    /// Remove the installation data for a specific widget
    /// # Returns
    /// The widget configuration
    pub fn remove_installation_data(widget_name: &str) {
        let config = Persistence::get_system_config();
        let Some(mut config) = config else {
            return;
        };
        if let Some(index) = config
            .widgets
            .iter()
            .position(|config: &WidgetInstallationData| config.name == widget_name)
        {
            config.widgets.swap_remove(index);
        }
        Persistence::save_system_config(config);
    }

    /// Returns Some system configuration if a new one is available
    /// Can be used for polling updates to the system configuration
    /// # Returns
    /// The system configuration if a new one is available
    pub fn get_system_config_change() -> Option<SystemConfiguration> {
        if CONFIG_UPDATED.load(Ordering::Relaxed) {
            CONFIG_UPDATED.store(false, Ordering::Relaxed);
            Some(Persistence::get_system_config().expect("Could not load config"))
        } else {
            None
        }
    }

    /// Save binary data to the database
    /// # Arguments
    /// * `key` - The key to save the data under
    /// * `bytes` - The data to save
    pub fn save_binary(key: &str, bytes: &[u8]) {
        DB.insert(key, bytes).expect("Could not save binary");
        CONFIG_UPDATED.store(true, Ordering::Relaxed);
    }

    /// Remove binary data from the database
    /// # Arguments
    /// * `key` - The key to remove
    pub fn remove_binary(key: &str) {
        DB.remove(key).expect("Could not remove binary");
        CONFIG_UPDATED.store(true, Ordering::Relaxed);
    }

    // Load binary data from the database
    // # Arguments
    // * `key` - The key to load
    // # Returns
    // The binary data
    pub fn get_binary(key: &str) -> Option<Vec<u8>> {
        let bytes = DB.get(key).expect("Could not read binary");
        bytes.map(|bytes| bytes.to_vec())
    }

    /// Create a default system configuration
    /// This is used on systems that never stored a configuration before
    fn create_default_system_config() {
        Persistence::save_system_config(SystemConfiguration::default());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_persistence() {
        let config = SystemConfiguration {
            background_color: "#FF3A3A".to_string(),
            widgets: vec![],
        };
        Persistence::save_system_config(config.clone());
        let read_config = Persistence::get_system_config();
        assert!(read_config.is_some());
        assert_eq!(config, read_config.unwrap());
    }
}
