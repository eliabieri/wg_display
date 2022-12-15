use async_trait::async_trait;
use common::models::WidgetConfiguration;
use common::widget_meta_data::WidgetMetaData;

#[async_trait]
pub trait Widget {
    fn new() -> Self
    where
        Self: Sized;
    fn get_meta_data(&self) -> WidgetMetaData;
    fn get_content(&self) -> &str;

    async fn update(&mut self, config: &WidgetConfiguration);
}
