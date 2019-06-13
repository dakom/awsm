use awsm::webgl::{ClearBufferMask, WebGlRenderer};
use awsm::window;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document, HtmlElement, HtmlCanvasElement};
use gloo_events::{EventListener};
use std::rc::Rc;
use std::cell::RefCell;

pub fn start_webgl<ResizeCb>(window: Window, document: Document, body: HtmlElement, mut resize_cb: ResizeCb) -> Result<Rc<RefCell<WebGlRenderer<'static>>>, JsValue> 
where ResizeCb: (FnMut(u32, u32) -> ()) + 'static,
{

    let canvas:HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    body.append_child(&canvas)?;

    let webgl_renderer = WebGlRenderer::new(canvas)?;
    let webgl_renderer = Rc::new(RefCell::new(webgl_renderer));
   
    let window_clone = window.clone();
    let webgl_renderer_clone = Rc::clone(&webgl_renderer);
    let mut on_resize = move |_:&web_sys::Event| {
        let (width, height) = window::get_size(&window_clone).unwrap();
        webgl_renderer_clone.borrow_mut().resize(width, height);
        resize_cb(width, height);
    };

    on_resize(&web_sys::Event::new("").unwrap());

    {
        let webgl_renderer = &webgl_renderer.borrow_mut();


        webgl_renderer.gl.clear_color(0.3, 0.3, 0.3, 1.0);
        webgl_renderer.clear(&[ClearBufferMask::ColorBufferBit, ClearBufferMask::DepthBufferBit]);
    }



    EventListener::new(&window, "resize",on_resize).forget();
    Ok(webgl_renderer)
}


