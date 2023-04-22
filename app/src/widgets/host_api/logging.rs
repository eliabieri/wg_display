use crate::widgets::running::runtime::{
    logging::{Host, Level},
    PluginState,
};

impl Host for PluginState {
    fn log(&mut self, level: Level, context: String, message: String) -> wasmtime::Result<()> {
        const PREFIX: &str = "PLUGIN";
        match level {
            Level::Debug => log::debug!("{}: {}: {}", PREFIX, context, message),
            Level::Info => log::info!("{}: {}: {}", PREFIX, context, message),
            Level::Warn => log::warn!("{}: {}: {}", PREFIX, context, message),
            Level::Error => log::error!("{}: {}: {}", PREFIX, context, message),
        }
        Ok(())
    }
}
