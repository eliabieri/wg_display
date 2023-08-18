use crate::widgets::running::runtime::widget::widget::logging;
use crate::widgets::running::runtime::WidgetState;

impl logging::Host for WidgetState {
    fn log(
        &mut self,
        level: logging::Level,
        context: String,
        message: String,
    ) -> wasmtime::Result<()> {
        const PREFIX: &str = "WIDGET";
        match level {
            logging::Level::Debug => log::debug!("{}: {}: {}", PREFIX, context, message),
            logging::Level::Info => log::info!("{}: {}: {}", PREFIX, context, message),
            logging::Level::Warn => log::warn!("{}: {}: {}", PREFIX, context, message),
            logging::Level::Error => log::error!("{}: {}: {}", PREFIX, context, message),
        }
        Ok(())
    }
}
