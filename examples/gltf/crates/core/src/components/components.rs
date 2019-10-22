pub use shared::events::{WindowSize};

pub struct InitState {
}

impl InitState {
    pub fn new () -> Self {
        Default::default()
    }
}

impl Default for InitState {
    fn default() -> Self {
        Self {
        }
    }
}
