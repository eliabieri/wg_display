use std::time::{Duration, Instant};

use super::base::Widget;
use common::models::WidgetConfiguration;
use common::widgets::WidgetName;
use serde::Deserialize;

#[derive(Deserialize)]
struct BernaquaData {
    #[serde(rename = "currentVisitors")]
    current_visitors: u32,
    #[serde(rename = "maxVisitors")]
    max_visitors: u32,
}

impl BernaquaData {
    fn get_capacity(&self) -> f32 {
        self.current_visitors as f32 / self.max_visitors as f32 * 100.0
    }
}

pub struct Bernaqua {
    content: String,
    last_updated: Instant,
}

#[async_trait]
impl Widget for Bernaqua {
    fn new() -> Self {
        Self {
            content: "Loading...".to_string(),
            last_updated: Instant::now()
                .checked_sub(Duration::from_secs(300))
                .unwrap(),
        }
    }

    fn get_name(&self) -> WidgetName {
        WidgetName::Bernaqua
    }

    fn get_content(&self) -> &str {
        self.content.as_str()
    }

    async fn update(&mut self, _config: &WidgetConfiguration) {
        if (self.last_updated.elapsed().as_secs()) < 60 {
            return;
        }

        const URL: &str = "https://blfa-api.migros.ch/fp/api/center/63/currentuser";
        let response = reqwest::get(URL).await;
        match response {
            Ok(response) => match response.json::<BernaquaData>().await {
                Ok(data) => {
                    self.content = format!("{:.1}% occupied", data.get_capacity());
                    self.last_updated = Instant::now();
                }
                Err(e) => {
                    self.content = format!("Could not deserialize data: {}", e);
                    self.last_updated = Instant::now();
                }
            },
            Err(error) => {
                self.content = format!("Could not update data: {}", error);
            }
        }
    }
}
