use wasm_bindgen::prelude::*;
use log::{info};
use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryInto;
use shared::events::{IoEventIndex, Timestamp};
use shared::state::State;
use crate::systems;
use crate::components::*;
use shipyard::*;

//if result is Ok(true) then send the updated state back
pub fn handle_event(evt_type:u32, evt_data: JsValue, world:&World, state:&mut State) -> Result<bool, JsValue> 
{
    let evt_type:IoEventIndex = evt_type.try_into()?;

    match evt_type {
        IoEventIndex::LoopBegin =>
        {
            let (timestamp, delta):(f64, f64) = serde_wasm_bindgen::from_value(evt_data)?;
            //info!("{} {}", timestamp, delta);
        },
        IoEventIndex::LoopUpdate =>
        {
            let delta:f64 = serde_wasm_bindgen::from_value(evt_data)?;
            systems::motion::update_motion(&world, delta);
            systems::state::extract_state(&world,state);
            return Ok(true);
            //info!("{}", delta);
        },
        /*
        IoEventIndex::LoopDraw =>
        {
            let interpolation:f64 = serde_wasm_bindgen::from_value(evt_data)?;
            //info!("{}", interpolation);
        },
        */
        IoEventIndex::LoopEnd=>
        {
            let (fps, end):(f64, bool) = serde_wasm_bindgen::from_value(evt_data)?;
            //info!("{} {}", fps, end);
        },

        IoEventIndex::WindowSize =>
        {
            let window_size:WindowSize = serde_wasm_bindgen::from_value(evt_data)?;
            world.run::<(EntitiesMut, &mut WindowSize), _>(|(mut entities, mut w)| {
                if let Some(w) = w.iter().next() {
                    w.width = window_size.width;
                    w.height = window_size.height;
                    //info!("got window size: {:?}", w);
                }
            });
        },

        IoEventIndex::LoadGltf => {
            let (base_path, filename):(String,String) = serde_wasm_bindgen::from_value(evt_data)?;

            info!("loading {}/{}", base_path, filename);
        },
        _ => 
        {
            info!("unknown event!");
        }
    }

    Ok(false)
}