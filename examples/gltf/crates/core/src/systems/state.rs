use shipyard::*;
use shared::state::*;
use log::{info};
use crate::components::*;

pub fn extract_state(world:&World, state:&mut State) {
    world.run::<(&WindowSize), _>(|(window_size)| {
        if let Some(window_size) = window_size.iter().next() {
            state.window_width = window_size.width;
            state.window_height = window_size.height;
        }
    });
}