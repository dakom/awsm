use awsm_web::webgl::{
    get_webgl_context_1, get_webgl_context_2, ClearBufferMask, WebGl1Renderer, WebGl2Renderer,
    WebGlContextOptions,
};

use awsm_web::tick::{TimestampLoop};
use awsm_web::window;
use gloo_events::EventListener;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Document, HtmlCanvasElement, HtmlElement, Window,
};

//Just call start_webgl! with the version and args
//Seems like a nice generic solution and works great for demos
//However, the implementation below is ridiculous, hehe...

pub enum WebGlVersion {
    One,
    Two,
}

#[macro_export]
macro_rules! start_webgl {
    ($version:ident, $window:ident, $document:ident, $body:ident, $setup_cb:expr, $resize_cb:expr, $tick_cb:expr) => {
        match $version {
            WebGlVersion::One => {
                start_webgl_1($window, $document, $body, $setup_cb, $resize_cb, $tick_cb)
            }
            WebGlVersion::Two => {
                start_webgl_2($window, $document, $body, $setup_cb, $resize_cb, $tick_cb)
            }
        }
    };
}

/*
 * Yeah - it's terrible
 * Besides the types, the *only* difference is registering extensions
 * Real-world though, an app is not really going to need both webgl1 and webgl2
 *
 * Different targets would really be different compilation altogether
 * But for the demo here we gotta support/test both - especially for core features
 */
pub fn start_webgl_1<ResizeCb, SetupCb, TickCb>(
    window: Window,
    document: Document,
    body: HtmlElement,
    setup_cb: SetupCb,
    mut resize_cb: ResizeCb,
    mut tick_cb: TickCb,
) -> Result<(), JsValue>
where
    SetupCb:
        (FnOnce(Rc<RefCell<WebGl1Renderer>>, Box<dyn FnOnce()>) -> Result<(), JsValue>) + 'static,
    ResizeCb: (FnMut(u32, u32) -> ()) + 'static,
    TickCb: (FnMut(f64, &mut WebGl1Renderer) -> ()) + 'static,
{
    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    body.append_child(&canvas)?;

    let gl = get_webgl_context_1(
        &canvas,
        Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        }),
    )?;

    let mut webgl_renderer: WebGl1Renderer =
        awsm_web::webgl::WebGlRenderer::new(gl).map_err(|err| JsValue::from_str(&err.to_string()))?;

    webgl_renderer
        .register_extension_instanced_arrays()
        .map_err(|err| JsValue::from_str(err.to_string().as_ref()))?;

    webgl_renderer
        .register_extension_vertex_array()
        .map_err(|err| JsValue::from_str(err.to_string().as_ref()))?;

    webgl_renderer
        .register_extension_vertex_array()
        .map_err(|err| JsValue::from_str(err.to_string().as_ref()))?;

    let webgl_renderer = Rc::new(RefCell::new(webgl_renderer));

    let webgl_renderer_clone = Rc::clone(&webgl_renderer);

    let window_clone = window.clone();

    let mut on_resize = move |_: &web_sys::Event| {
        let (width, height) = window::get_window_size(&window_clone).unwrap();
        webgl_renderer_clone.borrow_mut().resize(width, height);
        resize_cb(width, height);
    };

    on_resize(&web_sys::Event::new("").unwrap());

    {
        let mut webgl_renderer = webgl_renderer.borrow_mut();

        webgl_renderer.set_clear_color(0.3, 0.3, 0.3, 1.0);
        webgl_renderer.clear(&[
            ClearBufferMask::ColorBufferBit,
            ClearBufferMask::DepthBufferBit,
        ]);
    }

    let on_setup_completed = Box::new({
        let webgl_renderer = Rc::clone(&webgl_renderer);
        move || {
            let tick_loop = TimestampLoop::start({
                move |timestamp| {
                    let mut webgl_renderer = webgl_renderer.borrow_mut();
                    tick_cb(timestamp.delta, &mut webgl_renderer)
                }
            })
            .unwrap();

            EventListener::new(&window, "resize", on_resize).forget();

            std::mem::forget(Box::new(tick_loop));
        }
    });
    setup_cb(Rc::clone(&webgl_renderer), on_setup_completed)?;

    Ok(())
}

pub fn start_webgl_2<ResizeCb, SetupCb, TickCb>(
    window: Window,
    document: Document,
    body: HtmlElement,
    setup_cb: SetupCb,
    mut resize_cb: ResizeCb,
    mut tick_cb: TickCb,
) -> Result<(), JsValue>
where
    SetupCb:
        (FnOnce(Rc<RefCell<WebGl2Renderer>>, Box<dyn FnOnce()>) -> Result<(), JsValue>) + 'static,
    ResizeCb: (FnMut(u32, u32) -> ()) + 'static,
    TickCb: (FnMut(f64, &mut WebGl2Renderer) -> ()) + 'static,
{
    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    body.append_child(&canvas)?;

    let gl = get_webgl_context_2(
        &canvas,
        Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        }),
    )?;

    let webgl_renderer: WebGl2Renderer =
        awsm_web::webgl::WebGlRenderer::new(gl).map_err(|err| JsValue::from_str(&err.to_string()))?;

    let webgl_renderer = Rc::new(RefCell::new(webgl_renderer));

    let webgl_renderer_clone = Rc::clone(&webgl_renderer);

    let window_clone = window.clone();

    let mut on_resize = move |_: &web_sys::Event| {
        let (width, height) = window::get_window_size(&window_clone).unwrap();
        webgl_renderer_clone.borrow_mut().resize(width, height);
        resize_cb(width, height);
    };

    on_resize(&web_sys::Event::new("").unwrap());

    {
        let mut webgl_renderer = webgl_renderer.borrow_mut();

        webgl_renderer.set_clear_color(0.3, 0.3, 0.3, 1.0);
        webgl_renderer.clear(&[
            ClearBufferMask::ColorBufferBit,
            ClearBufferMask::DepthBufferBit,
        ]);
    }

    let on_setup_completed = Box::new({
        let webgl_renderer = Rc::clone(&webgl_renderer);
        move || {
            let tick_loop = TimestampLoop::start({
                move |timestamp| {
                    let mut webgl_renderer = webgl_renderer.borrow_mut();
                    tick_cb(timestamp.delta, &mut webgl_renderer)
                }
            })
            .unwrap();

            EventListener::new(&window, "resize", on_resize).forget();

            std::mem::forget(Box::new(tick_loop));
        }
    });
    setup_cb(Rc::clone(&webgl_renderer), on_setup_completed)?;

    Ok(())
}
