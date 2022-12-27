//! Shows the current date and time
use super::base::Widget;
use async_trait::async_trait;
use common::models::WidgetConfiguration;
use common::widget_meta_data::WidgetMetaData;

extern crate chrono;
use chrono::Local;

pub struct Today {
    content: String,
}

#[async_trait]
impl Widget for Today {
    fn new() -> Self {
        Self {
            content: "Loading...".to_string(),
        }
    }

    fn get_meta_data(&self) -> common::widget_meta_data::WidgetMetaData {
        WidgetMetaData::Today
    }

    fn get_content(&self) -> &str {
        self.content.as_str()
    }

    async fn update(&mut self, _config: &WidgetConfiguration) {
        let date = Local::now();
        self.content = date.format("%a, %e. %b  %H:%M:%S").to_string();
    }
}
