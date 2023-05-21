use crate::models::WidgetStoreItem;

impl WidgetStoreItem {
    pub fn new(name: String, description: String, repository: String) -> Self {
        Self {
            name,
            description,
            repository,
        }
    }

    /// Get the download url for the latest release of the widget
    /// This information is derived from the repository url
    /// # Returns
    /// The download url
    pub fn get_download_url(&self) -> String {
        const SUFFIX: &str = "releases/latest/download/widget.wasm";
        if self.repository.ends_with('/') {
            return format!("{}{}", self.repository, SUFFIX);
        }
        format!("{}/{}", self.repository, SUFFIX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_url() {
        let item = WidgetStoreItem::new(
            "My Widget".to_string(),
            "A widget for doing things".to_string(),
            "https://github.com/myusername/mywidget".to_string(),
        );
        let expected_url =
            "https://github.com/myusername/mywidget/releases/latest/download/widget.wasm"
                .to_string();
        assert_eq!(item.get_download_url(), expected_url);
    }
}
