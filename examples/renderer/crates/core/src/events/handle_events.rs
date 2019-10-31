use wasm_bindgen::prelude::*;
use log::{info};
use std::convert::TryInto;
use awsm_renderer::Renderer;
use super::events::*;
use super::event_sender::EventSender;
use super::{BridgeEventIndex};
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::futures_0_3::future_to_promise;
//if result is Ok(true) then send the updated state back

pub fn handle_event(evt_type:u32, evt_data: JsValue, renderer:Rc<RefCell<Renderer>>, event_sender:Rc<EventSender>) -> Result<(), JsValue> 
{
    let evt_type:BridgeEventIndex = evt_type.try_into()?;

    match evt_type {
        BridgeEventIndex::WindowSize =>
        {
            let window_size:WindowSize = serde_wasm_bindgen::from_value(evt_data)?;
            renderer.borrow_mut().resize(window_size.width, window_size.height);
            /*

            world.run::<(&mut WindowSize), _>(|w| {
                if let Some(w) = w.iter().next() {
                    w.width = window_size.width;
                    w.height = window_size.height;
                }
            });
            */
        },

        BridgeEventIndex::LoadGltf =>
        {
            let gltf_path:String = serde_wasm_bindgen::from_value(evt_data)?;
            future_to_promise({
                async move {
                    info!("loading {}", gltf_path);
                    let res = renderer.borrow_mut().load_gltf(&gltf_path).await?;
                    event_sender.send(BridgeEventIndex::GltfLoaded);
                    Ok(JsValue::null())
                }
            });

        },

        _ => 
        {
            info!("unknown event!");
        }
    }

    Ok(())
}