use crate::widgets::running::runtime::Runtime;
use anyhow::Error;

use super::persistence::Persistence;

pub struct WidgetManager {}

impl WidgetManager {
    pub async fn install_widget(download_url: &str, description: &str) -> Result<(), Error> {
        let response = reqwest::get(download_url).await?;
        let bytes = response.bytes().await?.to_vec();

        let mut runtime = Runtime::new();
        let widget = runtime.instantiate_widget(&bytes)?;
        let widget_name = runtime.get_widget_name(&widget)?;
        Persistence::save_binary(widget_name.as_str(), &bytes);

        if Persistence::get_widget_config(widget_name.as_str()).is_none() {
            Persistence::add_widget_default_config(widget_name.as_str(), description);
        }

        Ok(())
    }

    pub async fn deinstall_widget(widget_name: &str) -> Result<(), Error> {
        Persistence::remove_binary(widget_name);
        Persistence::remove_widget_config(widget_name);
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
            let bytes = Persistence::get_binary(&widget.name);
            if let Some(b) = bytes {
                widgets.push(b);
            };
        }
        widgets
    }
}
