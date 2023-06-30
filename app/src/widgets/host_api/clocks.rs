use std::time::UNIX_EPOCH;

use crate::widgets::running::runtime::widget::widget::clocks;
use crate::widgets::running::runtime::WidgetState;

impl clocks::Host for WidgetState {
    fn now(&mut self) -> wasmtime::Result<clocks::Datetime> {
        let now = std::time::SystemTime::now();
        let now = now.duration_since(UNIX_EPOCH).unwrap();
        Ok(clocks::Datetime {
            seconds: now.as_secs(),
            nanoseconds: now.subsec_nanos(),
        })
    }
}
