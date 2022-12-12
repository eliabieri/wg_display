use std::rc::Rc;

use gloo_console::log;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::functional::Reducible;

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct PublicTransportConfig {
    pub base_config: BaseWidgetConfig,
    pub from: String,
    pub to: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct BaseWidgetConfig {
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct WidgetConfiguration {
    pub time_config: BaseWidgetConfig,
    pub aare_config: BaseWidgetConfig,
    pub cafete_config: BaseWidgetConfig,
    pub bernaqua_config: BaseWidgetConfig,
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

#[derive(PartialEq)]
pub enum SystemConfigurationAction {
    SetInitialConfig(SystemConfiguration),
    SetTimeConfig(BaseWidgetConfig),
    SetAareConfig(BaseWidgetConfig),
    SetCafeteConfig(BaseWidgetConfig),
    SetBernaquaConfig(BaseWidgetConfig),
    SetPublicTransportBaseConfig(BaseWidgetConfig),
    SetPublicTransportConfig(PublicTransportConfig),
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
            SystemConfigurationAction::SetPublicTransportBaseConfig(widget_config) => Self {
                widget_config: WidgetConfiguration {
                    public_transport_config: PublicTransportConfig {
                        base_config: widget_config,
                        ..self.widget_config.public_transport_config.clone()
                    },
                    ..self.widget_config.clone()
                },
                ..(*self).clone()
            },
            SystemConfigurationAction::SetPublicTransportConfig(widget_config) => Self {
                widget_config: WidgetConfiguration {
                    public_transport_config: widget_config,
                    ..self.widget_config.clone()
                },
                ..(*self).clone()
            },
        };
        persist_system_config(config.clone());
        config.into()
    }
}
