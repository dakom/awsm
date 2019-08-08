use crate::errors::{Error};
use specs::{World, WorldExt};
use web_sys::{ AudioContext };

pub fn setup_audio( world:&mut World, audio_context: &AudioContext) -> Result<(), Error> {
    Ok(())
}
