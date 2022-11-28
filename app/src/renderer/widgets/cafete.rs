use std::time::{Duration, Instant};

use super::base::Widget;
use common::models::WidgetConfiguration;
use common::widgets::WidgetName;
use serde::Deserialize;

#[derive(Deserialize)]
struct CafeteData {
    values: Vec<Vec<String>>,
}

impl CafeteData {
    fn get_lineup(&self) -> String {
        let lines = self
            .values
            .iter()
            .map(|line| line.join(" "))
            .collect::<Vec<String>>();
        lines[1..3].join("\n")
    }
}

pub struct Cafete {
    content: String,
    last_updated: Instant,
}

#[async_trait]
impl Widget for Cafete {
    fn new() -> Self {
        Self {
            content: "Loading...".to_string(),
            last_updated: Instant::now()
                .checked_sub(Duration::from_secs(300))
                .unwrap(),
        }
    }

    fn get_name(&self) -> WidgetName {
        WidgetName::Cafete
    }

    fn get_content(&self) -> &str {
        self.content.as_str()
    }

    async fn update(&mut self, _config: &WidgetConfiguration) {
        if (self.last_updated.elapsed().as_secs()) < 300 {
            return;
        }

        const URL: &str = "https://sheets.googleapis.com/v4/spreadsheets/1RHBW-MrQHf79m__ULvr2NB7rGHKEhHR0M8hD620aU0o/values/Data?key=AIzaSyCcXzibzeMK37jNVDAWooNDiSs2H4IJs_c";
        let response = reqwest::get(URL).await;
        match response {
            Ok(response) => match response.json::<CafeteData>().await {
                Ok(data) => {
                    self.content = data.get_lineup();
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
