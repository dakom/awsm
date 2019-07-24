use crate::WebGlRenderer;
use awsm::webgl::{get_webgl_context_1, get_webgl_context_2, ClearBufferMask, WebGlContextOptions};
use awsm::window;
use gloo_events::EventListener;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Document, HtmlCanvasElement, HtmlElement, WebGl2RenderingContext, WebGlRenderingContext, Window,
};

#[cfg(feature = "webgl_1")]
fn get_renderer(
    canvas: HtmlCanvasElement,
    opts: Option<&WebGlContextOptions>,
) -> Result<WebGlRenderer, JsValue> {
    let gl = get_webgl_context_1(&canvas, opts)?;
    awsm::webgl::WebGlRenderer::new(gl).map_err(|err| err.into())
}

#[cfg(feature = "webgl_2")]
fn get_renderer(
    canvas: HtmlCanvasElement,
    opts: Option<&WebGlContextOptions>,
) -> Result<WebGlRenderer, JsValue> {
    let gl = get_webgl_context_2(&canvas, opts)?;
    awsm::webgl::WebGlRenderer::new(gl).map_err(|err| err.into())
}

pub fn start_webgl<ResizeCb>(
    window: Window,
    document: Document,
    body: HtmlElement,
    mut resize_cb: ResizeCb,
) -> Result<Rc<RefCell<WebGlRenderer>>, JsValue>
where
    ResizeCb: (FnMut(u32, u32) -> ()) + 'static,
{
    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    body.append_child(&canvas)?;

    let webgl_renderer = get_renderer(
        canvas,
        Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        }),
    )?;

    let webgl_renderer = Rc::new(RefCell::new(webgl_renderer));
    let webgl_renderer_clone = Rc::clone(&webgl_renderer);

    let window_clone = window.clone();

    let mut on_resize = move |_: &web_sys::Event| {
        let (width, height) = window::get_size(&window_clone).unwrap();
        webgl_renderer_clone.borrow_mut().resize(width, height);
        resize_cb(width, height);
    };

    on_resize(&web_sys::Event::new("").unwrap());

    {
        let webgl_renderer = webgl_renderer.borrow_mut();

        webgl_renderer.gl.clear_color(0.3, 0.3, 0.3, 1.0);
        webgl_renderer.clear(&[
            ClearBufferMask::ColorBufferBit,
            ClearBufferMask::DepthBufferBit,
        ]);
    }

    EventListener::new(&window, "resize", on_resize).forget();
    Ok(webgl_renderer)
}
