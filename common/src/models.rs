use std::rc::Rc;

use gloo_console::log;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::functional::Reducible;

/// The base configuration for all widgets.
#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct BaseWidgetConfig {
    /// Whether the widget is enabled or not.
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct WidgetConfiguration {
    pub name: String,
    pub json_config: String,
}

/// The system configuration.
/// Stores all configuration that is not specific to one widget.
#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct SystemConfiguration {
    #[serde(default)]
    pub background_color: String,
    #[serde(default)]
    pub widget_config: Vec<WidgetConfiguration>,
}

/// Stores the data needed for a widget installation
#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct InstallationData {
    pub download_url: String,
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

/// Actions that can be dispatched to the system configuration reducer.
/// The reducer is responsible for updating and persisting the system configuration state.
#[derive(PartialEq)]
pub enum SystemConfigurationAction {
    SetInitialConfig(SystemConfiguration),
    SetBackgroundColor(String),
    AddWidget(WidgetConfiguration),
}

impl Reducible for SystemConfiguration {
    type Action = SystemConfigurationAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let config = match action {
            SystemConfigurationAction::SetInitialConfig(new_config) => new_config,
            SystemConfigurationAction::SetBackgroundColor(background_color) => Self {
                background_color,
                ..(*self).clone()
            },
            SystemConfigurationAction::AddWidget(new_config) => Self {
                widget_config: {
                    let mut widget_config = self.widget_config.clone();
                    widget_config.push(new_config);
                    widget_config
                },
                ..(*self).clone()
            },
        };
        persist_system_config(config.clone());
        config.into()
    }
}
