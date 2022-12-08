use std::time::{Duration, Instant};

use common::{models::WidgetConfiguration, widgets::WidgetName};
use serde::Deserialize;
use time::OffsetDateTime;
use time_humanize::{Accuracy, HumanTime, Tense};

use super::base::Widget;

#[derive(Deserialize)]
struct FromData {
    #[serde(with = "time::serde::iso8601")]
    departure: OffsetDateTime,
}

#[derive(Deserialize)]
struct ConnectionData {
    from: FromData,
}

#[derive(Deserialize)]
struct PublicTransportData {
    connections: Vec<ConnectionData>,
}

// -------------------------------------------------------

pub struct PublicTransport {
    content: String,
    last_updated: Instant,
    data: PublicTransportData,
}

#[async_trait]
impl Widget for PublicTransport {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            content: "Loading...".to_string(),
            last_updated: Instant::now()
                .checked_sub(Duration::from_secs(300))
                .unwrap(),
            data: PublicTransportData {
                connections: vec![],
            },
        }
    }

    fn get_name(&self) -> common::widgets::WidgetName {
        WidgetName::PublicTransport
    }

    fn get_content(&self) -> &str {
        &self.content
    }

    async fn update(&mut self, config: &WidgetConfiguration) {
        let config = &config.public_transport_config;
        if config.from.is_empty() || config.to.is_empty() {
            self.content = "From and to need to be specified!".to_string();
            return;
        }

        if !self.data.connections.is_empty() {
            let departure = self.data.connections[0].from.departure;
            let departure_offset = departure - OffsetDateTime::now_utc();
            let departure = HumanTime::from(departure_offset.unsigned_abs())
                .to_text_en(Accuracy::Rough, Tense::Future);
            if departure_offset.is_negative() {
                self.data.connections.remove(0);
            }
            self.content = format!("{} -> {}: {}", config.from, config.to, departure);
        } else {
            self.content = format!(
                "None upcoming.\nNext update in {} secs",
                300 - self.last_updated.elapsed().as_secs()
            );
        }

        if (self.last_updated.elapsed().as_secs()) < 300 {
            return;
        }

        let url = format!(
            "http://transport.opendata.ch/v1/connections?from={}&to={}&limit={}",
            urlencoding::encode(config.from.as_str()),
            urlencoding::encode(config.to.as_str()),
            3,
        );
        let response = reqwest::get(url).await;
        match response {
            Ok(response) => match response.json::<PublicTransportData>().await {
                Ok(data) => {
                    self.data = data;
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
