use crate::widgets::running::runtime::widget::widget::random;
use crate::widgets::running::runtime::WidgetState;
use rand::prelude::*;

fn get_random() -> u64 {
    let mut rng = thread_rng();
    rng.next_u64()
}

impl random::Host for WidgetState {
    fn get_random(&mut self) -> u64 {
        get_random()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random() {
        let result = get_random();
        assert!(result > 0);
    }
}
