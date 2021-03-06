use wasm_bindgen::prelude::*;
use log::{info};
use std::convert::TryInto;
use awsm_renderer::Renderer;
use awsm_renderer::gltf::loader::{load_gltf};
use awsm_renderer::nodes::{NodeData};
use awsm_renderer::camera::{get_orthographic_projection, get_perspective_projection};
use awsm_renderer::transform::{Vector3, Matrix4, TransformValues, Quaternion};
use super::events;
use super::event_sender::EventSender;
use super::{BridgeEvent};
use crate::state::{CameraSettings, OrthographicCamera, PerspectiveCamera};
use crate::state::{State, self};
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen_futures::future_to_promise;
//if result is Ok(true) then send the updated state back

pub fn handle_event(evt_type:u32, evt_data: JsValue, state: Rc<RefCell<State>>, renderer:Rc<RefCell<Renderer>>, event_sender:Rc<EventSender>) -> Result<(), JsValue> 
{
    let evt_type:BridgeEvent = evt_type.try_into()?;

    match evt_type {
        BridgeEvent::WindowSize =>
        {
            let window_size:events::WindowSize = serde_wasm_bindgen::from_value(evt_data)?;
            state.borrow_mut().window_size = window_size;
            update_view(state, renderer)?;
        },

        BridgeEvent::LoadGltf =>
        {
            let filepath:String = serde_wasm_bindgen::from_value(evt_data)?;

            future_to_promise({
                async move {
                    let resource = load_gltf(&filepath, None).await?;
                    let mut renderer = renderer.borrow_mut();

                    //maybe upload_gltf should return entity list so it can be mixed in... 
                    renderer.upload_gltf(&resource, None)?;

                    renderer.set_scene_from_gltf(&resource.gltf);

                    event_sender.send(BridgeEvent::GltfLoaded);
                    Ok(JsValue::null())
                }
            });

        },

        BridgeEvent::CameraSettings =>
        {
            let camera_settings:events::CameraSettings = serde_wasm_bindgen::from_value(evt_data)?;
            match camera_settings.style.try_into()? {
                events::CameraStyle::Orthographic => {
                    state.borrow_mut().camera_settings = Some(CameraSettings::Orthographic(OrthographicCamera{
                        xmag: camera_settings.xmag.unwrap(),
                        ymag: camera_settings.ymag.unwrap(),
                        znear: camera_settings.znear.unwrap(),
                        zfar: camera_settings.zfar.unwrap(),
                        translation: Vector3::new( camera_settings.positionX,camera_settings.positionY,camera_settings.positionZ)
                    }));
                },
                events::CameraStyle::Perspective => {
                    state.borrow_mut().camera_settings = Some(CameraSettings::Perspective(PerspectiveCamera{
                        aspectRatio: camera_settings.aspectRatio.unwrap(),
                        yfov: camera_settings.yfov.unwrap(),
                        znear: camera_settings.znear.unwrap(),
                        zfar: camera_settings.zfar.unwrap(),
                        translation: Vector3::new( camera_settings.positionX,camera_settings.positionY,camera_settings.positionZ)
                    }));
                }
            };

            update_view(state, renderer)?;
        },
        BridgeEvent::Clear => 
        {
            info!("clearing all");
        }
        _ => 
        {
            info!("unknown event!");
        }
    }

    Ok(())
}

fn update_view(state: Rc<RefCell<State>>, renderer:Rc<RefCell<Renderer>>) -> Result<(), JsValue> {

    let mut renderer = renderer.borrow_mut();
    let mut state = state.borrow_mut();
    let events::WindowSize {width, height} = state.window_size;
    renderer.resize(width, height);

    if let Some((camera_translation, camera_projection)) = match &state.camera_settings {
        Some(CameraSettings::Orthographic(settings)) => {
            //scale ymag to keep things square with screen size
            let ratio = (height as f64) / (width as f64);
            let mut ymag = ratio * settings.ymag;

            //scale xmag and ymag by 2.0 just to fit content on screen probably
            let xmag = settings.xmag * 2.0;
            ymag *= 2.0;

            Some((settings.translation.clone(), get_orthographic_projection(xmag, ymag, settings.znear, settings.zfar)))
        },
        Some(CameraSettings::Perspective(settings)) => {
            Some((settings.translation.clone(), get_perspective_projection(settings.aspectRatio, settings.yfov, settings.znear, Some(settings.zfar))))
        },
        None => None
    } {
        //let rotation = Some(Quaternion::new(0.0, 1.0, 0.0, 0.0));
        let rotation = None;
        let translation = Some(camera_translation);
        if let Some(camera_node) = renderer.get_camera_node() {
            renderer.set_node_trs(camera_node, translation, rotation, None);
            renderer.update_camera_projection(None, camera_projection.as_ref());
        } else {
            renderer.add_node(NodeData::Camera(camera_projection), None, translation, rotation, None)?;
        } 
    }
    Ok(())
}