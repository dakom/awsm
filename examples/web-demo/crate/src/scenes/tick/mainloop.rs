use awsm_web::tick::{MainLoop, MainLoopOptions, RafLoop};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window};

const MAX: u64 = 10;

pub fn start(_window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {
    let root: Element = document.create_element("div")?.into();
    root.set_class_name("tick");
    body.append_child(&root)?;

    let header: web_sys::HtmlElement = document.create_element("h1")?.dyn_into()?;
    header.set_text_content(Some("Waiting for first tick"));
    root.append_child(&header)?;

    let begin_div: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
    root.append_child(&begin_div)?;

    let update_div: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
    root.append_child(&update_div)?;

    let interpolate_div: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
    root.append_child(&interpolate_div)?;

    let fps_div: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
    root.append_child(&fps_div)?;

    let mut elapsed = 0.0f64;

    //Closure needs to take ownership since it occurs past the JS boundry and is 'static
    //but we need to assign the value of cancel from outside the closure
    let raf_loop: Rc<RefCell<Option<RafLoop>>> = Rc::new(RefCell::new(None));

    //callbacks
    let begin = move |time, delta| {
        let my_str = format!("begin time: {} delta: {}!", time, delta);
        begin_div.set_text_content(Some(&my_str.as_str()));
    };

    let update = {
        let raf_loop = raf_loop.clone();
        move |delta| {
            elapsed += delta;
            let elapsed = (elapsed / 1000.0).round() as u64;
            let my_str = format!(
                "{} seconds left till stopping the ticker!",
                get_remaining(elapsed)
            );
            header.set_text_content(Some(&my_str.as_str()));
            let my_str = format!("updating timestep of {}!", delta);
            update_div.set_text_content(Some(&my_str.as_str()));

            if elapsed > MAX {
                raf_loop.borrow_mut().take();
                header.set_text_content(Some("ticker stopped!"));
            }
        }
    };

    let draw = move |interpolation| {
        let my_str = format!("interpolating {}!", interpolation);
        interpolate_div.set_text_content(Some(&my_str.as_str()));
    };
    let end = move |fps, abort| {
        let my_str = format!("fps: {} (aborting: {})!", fps, abort);
        fps_div.set_text_content(Some(&my_str.as_str()));
    };

    let _raf_loop = RafLoop::start({
        let mut main_loop = MainLoop::new(MainLoopOptions::default(), begin, update, draw, end);
        move |ts| {
            main_loop.tick(ts);
        }
    })?;

    *raf_loop.borrow_mut() = Some(_raf_loop);

    Ok(())
}

fn get_remaining(elapsed: u64) -> u64 {
    if MAX > elapsed {
        MAX - elapsed
    } else {
        0
    }
}
