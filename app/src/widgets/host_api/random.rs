use crate::widgets::running::runtime::{random::Host, WidgetState};
use rand::prelude::*;

impl Host for WidgetState {
    fn get_random(&mut self) -> wasmtime::Result<u64> {
        let mut rng = thread_rng();
        Ok(rng.next_u64())
    }
}
