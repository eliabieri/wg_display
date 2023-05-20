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

    /// Get all items in the store
    /// Use `fetch_from_store` to fetch the store before
    /// # Returns
    /// A vector of all items in the store
    pub fn get_items(&self) -> &Vec<WidgetStoreItem> {
        &self.store_items
    }

    /// Fetch the store from the internet
    /// # Returns
    /// An error if the fetch failed
    pub async fn fetch_from_store(&mut self) -> Result<()> {
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
        store.fetch_from_store().await.unwrap();
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
