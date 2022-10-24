use async_trait::async_trait;
use common::models::Configuration;

#[async_trait]
pub trait Widget {
    fn new() -> Self
    where
        Self: Sized;
    fn get_name(&self) -> &str;
    fn get_content(&self) -> &str;
    async fn update(&mut self, config: &Configuration);
}
