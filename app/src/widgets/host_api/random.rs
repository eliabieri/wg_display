use crate::widgets::running::runtime::widget::widget::random;
use crate::widgets::running::runtime::WidgetState;
use rand::prelude::*;

impl random::Host for WidgetState {
    fn get_random(&mut self) -> wasmtime::Result<u64> {
        let mut rng = thread_rng();
        Ok(rng.next_u64())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::running::runtime::widget::widget::random::Host;

    #[test]
    fn test_get_random() {
        let mut widget_state = WidgetState {};
        let result = widget_state.get_random().unwrap();
        assert!(result > 0);
    }
}
