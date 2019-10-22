use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::futures_0_3::future_to_promise;
use web_sys::{HtmlCanvasElement};
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use shared::state::*;
use super::events::*;
use super::assets::load_assets;
use nalgebra::{Matrix4, Point2, Vector2, Vector3};
use awsm_web::loaders::fetch;
use awsm_web::webgl::{
    get_webgl_context_2, 
    WebGlContextOptions, 
    ClearBufferMask,
    WebGlCommon,
    WebGl2Renderer,
    Id,
    GlToggle,
    BeginMode,
    BlendFactor
};

pub struct Renderer {
    pub webgl:WebGl2Renderer,
    pub program_id: Option<Id>,
    pub texture_id: Option<Id>,
    pub vao_id: Option<Id>,
    event_sender: EventSender,
    prev_state: Option<State>,
}

impl Renderer {
    pub fn new(canvas:HtmlCanvasElement, send_event: js_sys::Function) -> Result<Self, JsValue> {
        let gl = get_webgl_context_2(&canvas, Some(&WebGlContextOptions{
            alpha: false,
            ..WebGlContextOptions::default()
        }))?;

        let webgl = WebGl2Renderer::new(gl)?;

        let event_sender = EventSender::new(send_event);

        Ok(Self {
            event_sender,
            webgl,
            program_id: None,
            texture_id: None,
            vao_id: None,
            prev_state: None,
        })
    }

    pub fn send_event(&self, evt:&Event) {
        self.event_sender.send(evt);
    }

    pub fn pre_render(&mut self, window_width: u32, window_height: u32) {
        //These are checked in awsm to skip if it's the same as last tick
        self.webgl.resize(window_width, window_height);
        self.webgl.gl.clear_color(1.0, 1.0, 1.0, 1.0);
        self.webgl.clear(&[
            ClearBufferMask::ColorBufferBit,
            ClearBufferMask::DepthBufferBit,
        ]);
    }

    pub fn on_state(&mut self, state:State, interpolation:f64) {
        self.pre_render(state.window_width, state.window_height);

        let size_changed = match &self.prev_state {
            None => true,
            Some(prev_state) => state.window_height != prev_state.window_height || state.window_width != prev_state.window_width
        };

        if size_changed {
            //self.update_camera(state.window_width, state.window_height);
        }

        //self.render(&state, interpolation);

        self.prev_state = Some(state);
        //info!("ball radius: {}, position: {:?}", consts::ball.radius, state.ball_position);
    }

}

pub fn start(canvas:HtmlCanvasElement, window_width: u32, window_height: u32, send_event:js_sys::Function) -> Result<JsValue, JsValue> {
    let mut renderer = Renderer::new(canvas, send_event)?;
 
    renderer.pre_render(window_width, window_height);

    // renderer.send_event(&IoEvent::SetSpeed(Speed(0.3)));

    let renderer = Rc::new(RefCell::new(renderer));

    load_assets(Rc::clone(&renderer));

    //Create a function which allows JS to call us for rendering
    //We will need to get a handle and forget the Closure
    //See https://stackoverflow.com/a/53219594/784519
    let _render = Closure::wrap(Box::new({
        let renderer = Rc::clone(&renderer);
        move |data:JsValue, interpolation:f64| {
            {
                let state:Result<State, serde_wasm_bindgen::Error> = serde_wasm_bindgen::from_value(data);
                match state {
                    Ok(state) => {
                        let mut renderer = renderer.borrow_mut();
                        renderer.on_state(state, interpolation);
                    },
                    Err(reason) => info!("Error: {:?}", reason)
                }
            }
        }
    }) as Box<FnMut(JsValue, f64) -> ()>);

    let render = _render.as_ref().clone();
    _render.forget();

    //Return the event sender
    Ok(render)
}