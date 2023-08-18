//! Implementation of the system configuration persistence
use common::models::{SystemConfiguration, WidgetInstallationData};
use rocket::serde::json::serde_json;

use std::sync::atomic::{AtomicBool, Ordering};

use crate::widgets::running::runtime::CompiledWidget;

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
    // * `version` - Version of the widget
    pub fn add_default_installation_data(widget_name: &str, description: &str, version: &str) {
        let config = Persistence::get_system_config().unwrap();
        let mut widgets = config.widgets;
        widgets.push(WidgetInstallationData {
            name: widget_name.to_string(),
            description: description.to_owned(),
            version: version.to_owned(),
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

    /// Save precompiled widget to the database
    /// # Arguments
    /// * `key` - The key to save the data under
    /// * `compiled_widget` - The precompiled widget to save
    pub fn save_compiled_widget(key: &str, compiled_widget: &CompiledWidget) {
        let bytes =
            bincode::serialize(compiled_widget).expect("Could not serialize compiled widget");
        DB.insert(key, bytes)
            .expect("Could not save compiled widget");
        CONFIG_UPDATED.store(true, Ordering::Relaxed);
    }

    /// Remove compiled widget from the database
    /// # Arguments
    /// * `key` - The key to remove
    pub fn remove_compiled_widget(key: &str) {
        DB.remove(key).expect("Could not remove compiled_widget");
        CONFIG_UPDATED.store(true, Ordering::Relaxed);
    }

    // Get compiled widget from the database
    // # Arguments
    // * `key` - The key to load
    // # Returns
    // The compiled widget
    pub fn get_compiled_widget(key: &str) -> Option<CompiledWidget> {
        let bytes = DB.get(key).expect("Could not read binary");
        match bytes {
            Some(bytes) => {
                let compiled_widget =
                    bincode::deserialize(bytes.as_ref()).expect("Could not deserialize binary");
                Some(compiled_widget)
            }
            _ => None,
        }
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
    fn test_happy_day() {
        let config = SystemConfiguration {
            background_color: "#FF3A3A".to_string(),
            widgets: vec![],
        };
        Persistence::save_system_config(config.clone());
        let read_config = Persistence::get_system_config();
        assert!(read_config.is_some());
        assert_eq!(config, read_config.unwrap());
    }

    #[test]
    fn test_get_compiled_widget_not_found() {
        let key = "non_existent_key";
        let result = Persistence::get_compiled_widget(key);
        assert!(result.is_none());
    }

    #[test]
    fn test_create_default_system_config() {
        Persistence::create_default_system_config();
        let read_config = Persistence::get_system_config().unwrap();
        assert_eq!(read_config.background_color, "".to_string());
        assert!(read_config.widgets.is_empty());
    }
}
