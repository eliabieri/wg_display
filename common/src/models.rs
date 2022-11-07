use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct DefaultWidgetConfig {
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct WidgetConfiguration {
    pub time_config: DefaultWidgetConfig,
    pub aare_config: DefaultWidgetConfig,
    pub cafete_config: DefaultWidgetConfig,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct SystemConfiguration {
    #[serde(default)]
    pub ssid: String,
    #[serde(default)]
    pub password: String,
    pub widget_config: WidgetConfiguration,
}

impl Default for SystemConfiguration {
    fn default() -> Self {
        Self {
            ssid: "".to_string(),
            password: "".to_string(),
            widget_config: WidgetConfiguration {
                time_config: DefaultWidgetConfig { enabled: true },
                aare_config: DefaultWidgetConfig { enabled: true },
                cafete_config: DefaultWidgetConfig { enabled: true },
            },
        }
    }
}
