use std::time::{Duration, Instant};

use super::base::Widget;
use common::models::Configuration;
use serde::Deserialize;

#[derive(Deserialize)]
struct AareData {
    aare: AareCityData,
}

#[derive(Deserialize)]
struct AareCityData {
    temperature: i32,
    temperature_text: String,
}

pub struct Aare {
    content: String,
    last_updated: Instant,
}

#[async_trait]
impl Widget for Aare {
    fn new() -> Self {
        Self {
            content: "Loading...".to_string(),
            last_updated: Instant::now()
                .checked_sub(Duration::from_secs(300))
                .unwrap(),
        }
    }

    fn get_name(&self) -> &str {
        "Aare"
    }

    fn get_content(&self) -> &str {
        self.content.as_str()
    }

    async fn update(&mut self, _config: &Configuration) {
        if (self.last_updated.elapsed().as_secs()) < 300 {
            return;
        }

        const URL: &str = "http://aareguru.existenz.ch/currentV2.php?app=homeAnwendung?city=Bern";
        let response = reqwest::get(URL).await;
        match response {
            Ok(response) => match response.json::<AareData>().await {
                Ok(data) => {
                    self.content = data.aare.temperature_text;
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
