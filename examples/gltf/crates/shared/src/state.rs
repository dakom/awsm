use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub window_width: u32,
    pub window_height: u32,
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for State {
    fn default() -> Self {
        Self{
            window_width: 0,
            window_height: 0,
        }
    }
}