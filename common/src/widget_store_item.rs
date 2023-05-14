use crate::models::WidgetStoreItem;

impl WidgetStoreItem {
    pub fn new(name: String, description: String, repository: String) -> Self {
        Self {
            name,
            description,
            repository,
        }
    }

    pub fn get_download_url(&self) -> String {
        const SUFFIX: &str = "releases/latest/download/widget.wasm";
        if self.repository.ends_with('/') {
            return format!("{}{}", self.repository, SUFFIX);
        }
        format!("{}/{}", self.repository, SUFFIX)
    }
}
