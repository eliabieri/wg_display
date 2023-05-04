use std::time::SystemTime;

use crate::widgets::running::runtime::clocks::Datetime;

impl Datetime {
    pub fn now() -> Datetime {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Could not get system time");
        Datetime {
            seconds: now.as_secs(),
            nanoseconds: now.subsec_nanos(),
        }
    }
}
