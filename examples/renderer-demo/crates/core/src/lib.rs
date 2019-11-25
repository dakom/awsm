#[allow(clippy::module_inception)]
mod events;
#[allow(clippy::module_inception)]
mod game_loop;
#[allow(clippy::module_inception)]
mod state;

use cfg_if::cfg_if;
use log::{info, Level};
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::game_loop::GameLoop;
use crate::events::*;
use crate::state::*;
use awsm_renderer::{ Renderer};
use awsm_renderer::webgl::{
    get_webgl_context_2, 
    WebGlContextOptions, 
    WebGl2Renderer,
};
use web_sys::{HtmlCanvasElement};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging only during debug builds
cfg_if! {
    if #[cfg(all(feature = "console_log", debug_assertions))] {
        fn init_log() {
            use console_log;
            console_log::init_with_level(Level::Trace).expect("error initializing log");

            info!("log initiated");
        }
    } else {
        fn init_log() {}
    }
}

// enable panic hook only during debug builds
cfg_if! {
    if #[cfg(all(feature = "console_error_panic_hook", debug_assertions))] {
        fn init_panic() {
            console_error_panic_hook::set_once();
        }
    } else {
        fn init_panic() {}
    }
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run(canvas:HtmlCanvasElement, window_width: u32, window_height: u32, send_bridge_event:js_sys::Function) -> Result<JsValue, JsValue> {
    init_panic();
    init_log();


    let event_sender = Rc::new(EventSender::new(send_bridge_event));
    let webgl = get_webgl_context_2(&canvas, Some(&WebGlContextOptions{
            alpha: false,
            ..WebGlContextOptions::default()
    }))?;
    let webgl = WebGl2Renderer::new(webgl)?;

    webgl.gl.clear_color(0.5, 0.5, 0.5, 1.0);

    let renderer = Renderer::new(Rc::new(RefCell::new(webgl)), None, window_width, window_height)?;
    let renderer = Rc::new(RefCell::new(renderer));

    let game_loop = Box::new({
        let renderer = Rc::clone(&renderer);
        GameLoop::new(renderer)?
    });

    let state = Rc::new(RefCell::new(State{
        camera_node: None,
        camera_settings: None,
        window_size: WindowSize{
            width: window_width,
            height: window_height
        }
    }));

    //Create a function which allows JS to send us events ad-hoc
    //We will need to get a handle and forget the Closure
    //See https://stackoverflow.com/a/53219594/784519
    let _send_event = Box::new({
        move |evt_type:u32, data:JsValue| {
            let renderer = Rc::clone(&renderer);
            let event_sender = Rc::clone(&event_sender);
            let state = Rc::clone(&state);
            //The actual handling of events is in this function
            if let Err(reason) = handle_event(evt_type, data, state, renderer, event_sender) {
                info!("Error: {:?}", reason);
            }
        }
    }) as Box<dyn FnMut(u32, JsValue) -> ()>;

    let _send_event = Closure::wrap(_send_event);

    let send_event = _send_event.as_ref().clone();

    //forget the things that need to persist in memory 
    std::mem::forget(game_loop);
    _send_event.forget();

    Ok(send_event)
}