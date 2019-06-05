use awsm::webgl::{WebGlRenderer};
use awsm::window;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document, HtmlElement, HtmlCanvasElement, WebGl2RenderingContext};
use gloo_events::{EventListener};
use std::rc::Rc;
use std::cell::RefCell;

pub fn start(window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {

    let canvas:HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    body.append_child(&canvas)?;

    let webgl_renderer = WebGlRenderer::new(canvas)?;
    let webgl_renderer = Rc::new(RefCell::new(webgl_renderer));
   
    let window_clone = window.clone();
    let webgl_renderer_clone = Rc::clone(&webgl_renderer);
    let on_resize = move |_:&web_sys::Event| {
        let (width, height) = window::get_size(&window_clone).unwrap();
        webgl_renderer_clone.borrow_mut().resize(width, height);
    };

    on_resize(&web_sys::Event::new("").unwrap());
    let gl = &webgl_renderer.borrow_mut().gl;
    gl.clear_color(0.3, 0.3, 0.3, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT); 


    EventListener::new(&window, "resize",on_resize).forget();
    Ok(())
}


