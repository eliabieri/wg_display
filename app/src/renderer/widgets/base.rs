//! Base trait for all widgets
use async_trait::async_trait;
use common::models::WidgetConfiguration;
use common::widget_meta_data::WidgetMetaData;

/// Base trait for all widgets
/// Every widget must implement this trait
#[async_trait]
pub trait Widget {
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the meta data of the widget
    /// This is used to identify the widget on the display and on the frontend dashboard application
    fn get_meta_data(&self) -> WidgetMetaData;

    /// Returns the content of the widget
    /// Widgets may use newlines to display multiple lines
    fn get_content(&self) -> &str;

    /// Updates the widget content
    /// This method is called periodically by the renderer
    /// The widget must implement its own timeout logic to prevent unnecessary updates
    async fn update(&mut self, config: &WidgetConfiguration);
}
