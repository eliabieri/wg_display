use crate::widgets::running::runtime::widget::widget::clocks;
use crate::widgets::running::runtime::widget::widget::types::Host;
use crate::widgets::running::runtime::WidgetState;
use std::time::SystemTime;

impl Host for WidgetState {}

impl clocks::Datetime {
    pub fn now() -> clocks::Datetime {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Could not get system time");
        clocks::Datetime {
            seconds: now.as_secs(),
            nanoseconds: now.subsec_nanos(),
        }
    }
}
