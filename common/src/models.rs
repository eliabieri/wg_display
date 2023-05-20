use std::rc::Rc;

use gloo_console::log;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::functional::Reducible;

/// The installation data of a widget
#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct WidgetInstallationData {
    pub name: String,
    pub description: String,
    pub json_config: String,
}

/// The system configuration.
/// Stores all configuration that is not specific to one widget.
#[derive(Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub struct SystemConfiguration {
    #[serde(default)]
    pub background_color: String,
    #[serde(default)]
    pub widgets: Vec<WidgetInstallationData>,
}

/// Stores the data needed for a widget installation
#[derive(Deserialize, Serialize, Clone)]
pub enum InstallAction {
    FromUrl(String),
    FromStoreItemName(String),
}

/// Represents the information associated with a widget in the store.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct WidgetStoreItem {
    pub name: String,
    pub description: String,
    pub repository: String,
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
    AddWidget(WidgetInstallationData),
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
                widgets: {
                    let mut widget_config = self.widgets.clone();
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
