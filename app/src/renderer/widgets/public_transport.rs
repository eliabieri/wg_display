//! Shows the next public transport connections
use std::time::{Duration, Instant};

use common::models::WidgetConfiguration;
use common::widget_meta_data::WidgetMetaData;
use serde::Deserialize;
use time::{format_description, OffsetDateTime};
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

    fn get_meta_data(&self) -> common::widget_meta_data::WidgetMetaData {
        WidgetMetaData::PublicTransport
    }

    fn get_content(&self) -> &str {
        &self.content
    }

    async fn update(&mut self, config: &WidgetConfiguration) {
        let config = &config.public_transport_config;
        if config.from.is_empty() || config.to.is_empty() {
            self.content = "`from` and `to` need to be configured!".to_string();
            return;
        }

        self.update_departure_string(config.num_connections_to_show as usize);

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
                    self.data
                        .connections
                        .dedup_by(|a, b| a.from.departure == b.from.departure);
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
    fn update_departure_string(&mut self, num_departures: usize) {
        self.content = format!("{} -> {}", self.data.from.name, self.data.to.name);

        if self.data.connections.is_empty() {
            self.content += "\nNo departures";
            return;
        }

        let connections = self
            .data
            .connections
            .iter()
            .filter(|connection| {
                (connection.from.departure - OffsetDateTime::now_utc()).is_positive()
            })
            .take(num_departures);

        for connection in connections {
            let departure = connection.from.departure;
            self.content += &format!(
                "\n{} ({})",
                PublicTransport::format_departure_offset(departure),
                PublicTransport::format_departure(departure)
            )
            .to_string();
        }
    }

    fn format_departure(departure: OffsetDateTime) -> String {
        let format = format_description::parse("[hour]:[minute]").unwrap();
        match departure.format(&format) {
            Ok(departure) => departure,
            Err(e) => {
                format!("Could not format departure: {}", e)
            }
        }
    }

    fn format_departure_offset(departure: OffsetDateTime) -> String {
        let departure_offset = departure - OffsetDateTime::now_utc();
        HumanTime::from(departure_offset.unsigned_abs()).to_text_en(Accuracy::Rough, Tense::Future)
    }
}

#[cfg(test)]
mod tests {
    use common::models::{BaseWidgetConfig, PublicTransportConfig, WidgetConfiguration};

    use super::*;

    #[test]
    fn test_formatting() {
        let mut public_transport = PublicTransport::new();
        public_transport.last_updated = Some(Instant::now());
        public_transport.data = PublicTransportData {
            connections: vec![
                ConnectionData {
                    from: FromData {
                        departure: OffsetDateTime::now_utc() + Duration::from_secs(23 * 60 + 1),
                    },
                },
                ConnectionData {
                    from: FromData {
                        departure: OffsetDateTime::now_utc() + Duration::from_secs(120 * 60 + 1),
                    },
                },
            ],
            from: FromMetaData {
                name: "Bern".to_string(),
            },
            to: ToMetaData {
                name: "Basel".to_string(),
            },
        };

        let config = WidgetConfiguration {
            public_transport_config: PublicTransportConfig {
                base_config: BaseWidgetConfig { enabled: true },
                from: "Bern".to_string(),
                to: "Basel".to_string(),
                num_connections_to_show: 2,
            },
            ..Default::default()
        };
        tokio_test::block_on(public_transport.update(&config));

        assert!(public_transport.content.contains("Bern -> Basel"));
        assert!(public_transport.content.contains("\nin 23 minutes"));
        assert!(public_transport.content.contains("\nin 2 hours"));
    }

    #[test]
    fn test_formatting_with_no_departures() {
        let mut public_transport = PublicTransport::new();
        public_transport.last_updated = Some(Instant::now());
        public_transport.data = PublicTransportData {
            connections: vec![],
            from: FromMetaData {
                name: "Bern".to_string(),
            },
            to: ToMetaData {
                name: "Basel".to_string(),
            },
        };

        let config = WidgetConfiguration {
            public_transport_config: PublicTransportConfig {
                base_config: BaseWidgetConfig { enabled: true },
                from: "Bern".to_string(),
                to: "Basel".to_string(),
                num_connections_to_show: 2,
            },
            ..Default::default()
        };
        tokio_test::block_on(public_transport.update(&config));

        assert_eq!(public_transport.content, "Bern -> Basel\nNo departures");
    }

    #[test]
    fn test_from_to_not_configured() {
        let mut public_transport = PublicTransport::new();
        public_transport.last_updated = Some(Instant::now());
        public_transport.data = PublicTransportData {
            connections: vec![],
            from: FromMetaData {
                name: "Bern".to_string(),
            },
            to: ToMetaData {
                name: "Basel".to_string(),
            },
        };

        let config = WidgetConfiguration {
            public_transport_config: PublicTransportConfig {
                base_config: BaseWidgetConfig { enabled: true },
                from: "".to_string(),
                to: "".to_string(),
                num_connections_to_show: 2,
            },
            ..Default::default()
        };
        tokio_test::block_on(public_transport.update(&config));

        assert_eq!(
            public_transport.content,
            "`from` and `to` need to be configured!"
        );
    }
}
