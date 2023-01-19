//! Shows the current menu at Buffet Nord (https://www.buffetnord.ch/menu)
use std::time::{Duration, Instant};

use super::base::Widget;
use common::models::WidgetConfiguration;
use common::widget_meta_data::WidgetMetaData;
use scraper::{ElementRef, Html, Selector};

pub struct BuffetNord {
    content: String,
    last_updated: Option<Instant>,
}

impl BuffetNord {
    fn parse_dietary(menu_item: &ElementRef) -> Option<String> {
        let svg_selector = Selector::parse(r"img").unwrap();
        let svg_elem = menu_item.select(&svg_selector).next()?;
        let src_value = svg_elem.value().attr("src")?;
        match src_value {
            "/vegetarian.svg" => Some("veg".to_string()),
            "/vegan.svg" => Some("V".to_string()),
            _ => None,
        }
    }

    fn parse_title(menu_item: &ElementRef) -> Option<String> {
        let title_selector = Selector::parse(r"div.is-size-2").unwrap();
        let title = menu_item.select(&title_selector).next()?;
        Some(title.text().collect())
    }
    fn parse_response(html: &str) -> String {
        let document = Html::parse_document(html);
        let menu_item_selector =
            Selector::parse(r"#__layout > div > div.main > div > article > div > section").unwrap();

        let mut text = String::new();
        for menu_item in document.select(&menu_item_selector) {
            let title = BuffetNord::parse_title(&menu_item);
            let dietary = BuffetNord::parse_dietary(&menu_item);
            match title {
                Some(title) => {
                    text.push_str(format!("{} ({})\n", title, dietary.unwrap_or_default()).as_str())
                }
                None => continue,
            }
        }
        text
    }
}

#[async_trait]
impl Widget for BuffetNord {
    fn new() -> Self {
        Self {
            content: "Loading...".to_string(),
            last_updated: None,
        }
    }

    fn get_meta_data(&self) -> common::widget_meta_data::WidgetMetaData {
        WidgetMetaData::BuffetNord
    }

    fn get_content(&self) -> &str {
        self.content.as_str()
    }

    async fn update(&mut self, _config: &WidgetConfiguration) {
        if let Some(time) = self.last_updated {
            if time.elapsed() < Duration::from_secs(60 * 60 * 6) {
                return;
            }
        }

        let response = reqwest::get("https://www.buffetnord.ch/menu").await;
        match response {
            Ok(response) => match response.text().await {
                Ok(html) => self.content = BuffetNord::parse_response(&html),
                Err(_) => {
                    self.content = "Could not load menu".into();
                    return;
                }
            },
            Err(_) => {
                self.content = "Could not load menu".into();
                return;
            }
        }
    }
}
