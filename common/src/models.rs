use std::rc::Rc;

use gloo_console::log;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::functional::Reducible;

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct PublicTransportConfig {
    pub enabled: bool,
    pub from: String,
    pub to: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct DefaultWidgetConfig {
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct WidgetConfiguration {
    pub time_config: DefaultWidgetConfig,
    pub aare_config: DefaultWidgetConfig,
    pub cafete_config: DefaultWidgetConfig,
    pub bernaqua_config: DefaultWidgetConfig,
    pub public_transport_config: PublicTransportConfig,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct SystemConfiguration {
    #[serde(default)]
    pub ssid: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub widget_config: WidgetConfiguration,
}

fn persist_system_config(config: SystemConfiguration) {
    log!("Persisting new system config..");
    wasm_bindgen_futures::spawn_local(async move {
        Request::post("/config")
            .json(&config)
            .expect("Could not serialize config")
            .send()
            .await
            .expect("Could not transmit config");
    });
}

pub enum SystemConfigurationAction {
    SetInitialConfig(SystemConfiguration),
    SetTimeConfig(DefaultWidgetConfig),
    SetAareConfig(DefaultWidgetConfig),
    SetCafeteConfig(DefaultWidgetConfig),
    SetBernaquaConfig(DefaultWidgetConfig),
}

impl Reducible for SystemConfiguration {
    type Action = SystemConfigurationAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let config = match action {
            SystemConfigurationAction::SetInitialConfig(new_config) => new_config,
            SystemConfigurationAction::SetTimeConfig(widget_config) => Self {
                widget_config: WidgetConfiguration {
                    time_config: widget_config,
                    ..self.widget_config.clone()
                },
                ..(*self).clone()
            },
            SystemConfigurationAction::SetAareConfig(widget_config) => Self {
                widget_config: WidgetConfiguration {
                    aare_config: widget_config,
                    ..self.widget_config.clone()
                },
                ..(*self).clone()
            },
            SystemConfigurationAction::SetCafeteConfig(widget_config) => Self {
                widget_config: WidgetConfiguration {
                    cafete_config: widget_config,
                    ..self.widget_config.clone()
                },
                ..(*self).clone()
            },
            SystemConfigurationAction::SetBernaquaConfig(widget_config) => Self {
                widget_config: WidgetConfiguration {
                    bernaqua_config: widget_config,
                    ..self.widget_config.clone()
                },
                ..(*self).clone()
            },
        };
        persist_system_config(config.clone());
        config.into()
    }
}
