use crate::widgets::running::runtime::Runtime;
use anyhow::Error;

use super::persistence::Persistence;

pub struct WidgetManager {}

impl WidgetManager {
    /// Download and persist a widget
    /// Also adds a default configuration for the widget
    /// # Arguments
    /// * `download_url` - The URL to download the widget from
    /// * `description` - A description of the widget
    /// # Returns
    /// An error if the download or persisting failed
    pub async fn install_widget(download_url: &str, description: &str) -> Result<(), Error> {
        let response = reqwest::get(download_url).await?;
        let bytes = response.bytes().await?.to_vec();

        let mut runtime = Runtime::new();
        let widget = runtime.instantiate_widget(&bytes)?;
        let widget_name = runtime.get_widget_name(&widget)?;
        Persistence::save_binary(widget_name.as_str(), &bytes);

        if Persistence::get_widget_config(widget_name.as_str()).is_none() {
            Persistence::add_default_installation_data(widget_name.as_str(), description);
        }

        Ok(())
    }

    /// Deinstall a widget
    /// # Arguments
    /// * `widget_name` - The name of the widget to deinstall
    /// # Returns
    /// An error if the deinstallation failed
    pub async fn deinstall_widget(widget_name: &str) -> Result<(), Error> {
        Persistence::remove_binary(widget_name);
        Persistence::remove_installation_data(widget_name);
        Ok(())
    }

    /// Get a previously installed widget
    /// # Arguments
    /// * `widget_name` - The name of the widget to get
    /// # Returns
    /// The widget as bytes
    pub fn get_widget(widget_name: &str) -> Result<Vec<u8>, Error> {
        let bytes =
            Persistence::get_binary(widget_name).ok_or(anyhow::anyhow!("Could not get widget"))?;
        Ok(bytes)
    }

    /// Get all installed and configured widgets
    /// # Returns
    /// A vector of widgets as bytes
    pub fn get_widgets() -> Vec<Vec<u8>> {
        let mut widgets = Vec::new();
        for widget in Persistence::get_system_config().unwrap().widgets {
            let bytes = Persistence::get_binary(&widget.name);
            if let Some(b) = bytes {
                widgets.push(b);
            };
        }
        widgets
    }
}
