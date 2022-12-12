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
struct FromMetaData {
    name: String,
}

#[derive(Deserialize)]
struct ToMetaData {
    name: String,
}

#[derive(Deserialize)]
struct ConnectionData {
    from: FromData,
}

#[derive(Deserialize)]
struct PublicTransportData {
    connections: Vec<ConnectionData>,
    from: FromMetaData,
    to: ToMetaData,
}

// -------------------------------------------------------

pub struct PublicTransport {
    content: String,
    last_updated: Option<Instant>,
    data: PublicTransportData,
    update_interval: Duration,
}

#[async_trait]
impl Widget for PublicTransport {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            content: "Loading...".to_string(),
            last_updated: None,
            data: PublicTransportData {
                connections: vec![],
                from: FromMetaData {
                    name: "Loading...".to_string(),
                },
                to: ToMetaData {
                    name: "Loading...".to_string(),
                },
            },
            update_interval: Duration::from_secs(90),
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

        self.update_departure_string(3);

        if let Some(last_updated) = self.last_updated {
            if last_updated.elapsed() < self.update_interval {
                return;
            }
        }

        let url = format!(
            "http://transport.opendata.ch/v1/connections?from={}&to={}&limit=16",
            urlencoding::encode(config.from.as_str()),
            urlencoding::encode(config.to.as_str()),
        );
        let response = reqwest::get(url).await;
        match response {
            Ok(response) => match response.json::<PublicTransportData>().await {
                Ok(data) => {
                    self.data = data;
                    self.last_updated = Some(Instant::now());
                }
                Err(e) => {
                    self.content = format!("Could not deserialize data: {}", e);
                    self.last_updated = Some(Instant::now());
                }
            },
            Err(error) => {
                self.content = format!("Could not update data: {}", error);
            }
        }
    }
}

impl PublicTransport {
    fn update_departure_string(&mut self, count: usize) {
        self.content = format!("{} -> {}", self.data.from.name, self.data.to.name);

        let connections = self
            .data
            .connections
            .iter()
            .filter(|connection| {
                (connection.from.departure - OffsetDateTime::now_utc()).is_positive()
            })
            .take(count);

        for connection in connections {
            let departure = connection.from.departure;
            let departure_offset = departure - OffsetDateTime::now_utc();
            let departure = HumanTime::from(departure_offset.unsigned_abs())
                .to_text_en(Accuracy::Rough, Tense::Future);
            self.content += &format!("\n{}", departure).to_string();
        }
    }
}