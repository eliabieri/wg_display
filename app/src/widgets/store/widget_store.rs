use anyhow::Result;
use common::models::WidgetStoreItem;

const WIDGET_LISTING_URL: &str =
    "https://raw.githubusercontent.com/eliabieri/wg_display/feature/wasm_widget_support/widget_store.json";

pub struct WidgetStore {
    store_items: Vec<WidgetStoreItem>,
}

impl WidgetStore {
    pub fn new() -> Self {
        Self {
            store_items: Vec::new(),
        }
    }

    pub fn get_items(&self) -> &Vec<WidgetStoreItem> {
        &self.store_items
    }

    pub async fn update_store(&mut self) -> Result<()> {
        let response = reqwest::get(WIDGET_LISTING_URL).await?;
        let body = response.text().await?;
        self.store_items = serde_json::from_str::<Vec<WidgetStoreItem>>(&body)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_update_store() {
        let mut store = WidgetStore::new();
        store.update_store().await.unwrap();
        assert!(!store.get_items().is_empty());
        let reference_item = store
            .get_items()
            .iter()
            .find(|item: &&WidgetStoreItem| item.name == "Rust Widget Template")
            .unwrap();
        assert_eq!(
            reference_item.description,
            "The template project to create a widget using Rust"
        );
    }
}
