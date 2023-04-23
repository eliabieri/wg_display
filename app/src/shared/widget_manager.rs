use anyhow::Error;
use common::models::SystemConfiguration;

use crate::widgets::{running::runtime::Runtime, utils::loader::Loader};

use super::persistence::Persistence;

pub struct WidgetManager {}

impl WidgetManager {
    pub async fn install_widget(download_url: &str) -> Result<(), Error> {
        let response = reqwest::get(download_url).await?;
        let bytes = response.bytes().await?.to_vec();
        let component = Loader::load_core_module_as_component(bytes.as_slice())?;

        let mut runtime = Runtime::new();
        let plugin = runtime.instantiate_plugin(&component)?;
        let widget_name = runtime.get_plugin_name(&plugin)?;
        Persistence::save_binary(widget_name.as_str(), &component);
        Persistence::add_widget_default_config(widget_name.as_str());
        Ok(())
    }

    pub fn get_widget(widget_name: &str) -> Result<Vec<u8>, Error> {
        let bytes =
            Persistence::get_binary(widget_name).ok_or(anyhow::anyhow!("Could not get widget"))?;
        Ok(bytes)
    }

    pub fn get_widgets() -> Vec<Vec<u8>> {
        let mut widgets = Vec::new();
        for widget in Persistence::get_system_config().unwrap().widget_config {
            let bytes = Persistence::get_binary(&widget.name).unwrap();
            widgets.push(bytes);
        }
        widgets
    }
}
