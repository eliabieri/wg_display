use std::time::UNIX_EPOCH;

use crate::widgets::running::runtime::{
    clocks::{self, Datetime},
    WidgetState,
};

impl clocks::Host for WidgetState {
    fn now(&mut self) -> wasmtime::Result<Datetime> {
        let now = std::time::SystemTime::now();
        let now = now.duration_since(UNIX_EPOCH).unwrap();
        Ok(Datetime {
            seconds: now.as_secs(),
            nanoseconds: now.subsec_nanos(),
        })
    }
}
