use super::base::Widget;
use async_trait::async_trait;
use common::{models::Configuration, widgets::WidgetName};

extern crate chrono;
use chrono::Local;

pub struct Time {
    content: String,
}

#[async_trait]
impl Widget for Time {
    fn new() -> Self {
        Self {
            content: "Loading...".to_string(),
        }
    }

    fn get_name(&self) -> WidgetName {
        WidgetName::Time
    }

    fn get_content(&self) -> &str {
        self.content.as_str()
    }

    async fn update(&mut self, _config: &Configuration) {
        let date = Local::now();
        self.content = date.format("%H:%M:%S").to_string();
    }
}
