use wasm_bindgen::prelude::*;
use log::{info};
use std::convert::TryInto;
use awsm_renderer::Renderer;
use awsm_renderer::gltf::loader::{load_gltf};
use awsm_renderer::nodes::{NodeData};
use super::events::*;
use super::event_sender::EventSender;
use super::{BridgeEventIndex};
use crate::state::*;
use awsm_renderer::transform::{Vector3, Matrix4, TransformValues};
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen_futures::futures_0_3::future_to_promise;
//if result is Ok(true) then send the updated state back

pub fn handle_event(evt_type:u32, evt_data: JsValue, state: Rc<RefCell<State>>, renderer:Rc<RefCell<Renderer>>, event_sender:Rc<EventSender>) -> Result<(), JsValue> 
{
    let evt_type:BridgeEventIndex = evt_type.try_into()?;

    match evt_type {
        BridgeEventIndex::WindowSize =>
        {
            let window_size:WindowSize = serde_wasm_bindgen::from_value(evt_data)?;
            state.borrow_mut().window_size = window_size;
            update_view(state, renderer)?;
        },

        BridgeEventIndex::LoadGltf =>
        {
            let filepath:String = serde_wasm_bindgen::from_value(evt_data)?;

            future_to_promise({
                async move {
                    let resource = load_gltf(&filepath, None).await?;
                    let mut renderer = renderer.borrow_mut();

                    //maybe upload_gltf should return entity list so it can be mixed in... 
                    renderer.upload_gltf(&resource, None)?;

                    renderer.set_scene_from_gltf(&resource.gltf);

                    event_sender.send(BridgeEventIndex::GltfLoaded);
                    Ok(JsValue::null())
                }
            });

        },

        BridgeEventIndex::CameraSettings =>
        {
            let camera_settings:CameraSettings = serde_wasm_bindgen::from_value(evt_data)?;
            let camera_style:CameraStyle = camera_settings.style.try_into()?;
            state.borrow_mut().camera_style = camera_style;

            update_view(state, renderer)?;
        },
        _ => 
        {
            info!("unknown event!");
        }
    }

    Ok(())
}
fn update_view(state: Rc<RefCell<State>>, renderer:Rc<RefCell<Renderer>>) -> Result<(), JsValue> {

    let mut renderer = renderer.borrow_mut();

    log::info!("TODO - remove previous camera node, or update instead of add if exists!");

    let state = state.borrow();
    let WindowSize {width, height} = state.window_size;
    renderer.resize(width, height);
    match state.camera_style {
        CameraStyle::Orthographic => {

            /* TODO - pick up next time...

                First attempt to try and construct an orthographic projection based on SCREEN SIZE did not work...
                However, it seems that an orthographic projection based on the viewport (-1.0 -> 1.0) does work! 

                gotta figure out why the screen-based orthographic camera fails
            */

            //SCREEN BASED:
            let projection = Matrix4::new_from_slice(nalgebra::Matrix4::new_orthographic(
                    0.0,
                    width as f64,
                    0.0,
                    height as f64,
                    0.01,
                    1000.0,
            ).as_slice());

            //VIEWPORT BASED:
            let projection = Matrix4::new_from_slice(nalgebra::Matrix4::new_orthographic(
                    -1.0,
                    1.0,
                    -1.0,
                    1.0,
                    0.01,
                    1.0,
            ).as_slice());

            let camera_translation = Vector3::new(0.0, 0.0, -1.0);

            //should be able to do this, but first gotta add inverting in nodes.rs:
            //let camera_translation = Vector3::new(0.0, 0.0, 1.0);

            renderer.add_node(NodeData::Camera(projection), None, Some(camera_translation), None, None);
        },
        CameraStyle::Perspective => log::info!("TODO: perspective"),
    };

    /* Orthographic example
        let camera_mat = Matrix4::new_orthographic(
                    0.0,
                    camera_width as f32,
                    0.0,
                    camera_height as f32,
                    0.0,
                    1.0,
                );
    */

    /* Perspective example
        
        // Our camera looks toward the point (1.0, 0.0, 0.0).
        // It is located at (0.0, 0.0, 1.0).
        let eye = Point3::new(1000.0, 500.0, 1000.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y()).to_homogeneous();

        // A perspective projection.
        let projection = Perspective3::new(
            state.camera_width as f32 / state.camera_height as f32,
            std::f32::consts::PI / 2.0,
            1.0,
            3000.0,
        )
        .to_homogeneous();

        let camera = vec![view.as_slice(), projection.as_slice()].concat();
    */
    Ok(())
}