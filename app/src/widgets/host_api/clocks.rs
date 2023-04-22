use std::time::UNIX_EPOCH;

use crate::widgets::running::runtime::{
    clocks::{self, Datetime},
    PluginState,
};

impl clocks::Host for PluginState {
    fn now(&mut self) -> wasmtime::Result<Datetime> {
        let now = std::time::SystemTime::now();
        Ok(Datetime {
            seconds: now.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            // TODO: implement nanoseconds
            nanoseconds: 0,
        })
    }
}
