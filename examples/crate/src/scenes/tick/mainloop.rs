use awsm::tick;
use awsm::tick::{MainLoopOptions, start_main_loop};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Element, Document, HtmlElement};
use std::rc::Rc;
use std::cell::RefCell;
use log::{info};

const MAX:u64 = 10;

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
    let cancel:Rc<RefCell<Option<Box<dyn FnOnce() -> ()>>>> = Rc::new(RefCell::new(None));

    //callbacks
    let begin = move |time, delta| {
        let my_str = format!("begin time: {} delta: {}!", time, delta);
        begin_div.set_text_content(Some(&my_str.as_str()));
    };

    let update = {
        let cancel = cancel.clone();
        move |delta| {
            elapsed += delta;
            let elapsed = (elapsed / 1000.0).round() as u64;
            let my_str = format!("{} seconds left till stopping the ticker!", get_remaining(elapsed));
            header.set_text_content(Some(&my_str.as_str()));
            let my_str = format!("updating timestep of {}!", delta);
            update_div.set_text_content(Some(&my_str.as_str()));

            if elapsed > MAX { 
                if let Some(cb) = cancel.borrow_mut().take() {
                    cb();
                }
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

    let cancel_fn = start_main_loop(MainLoopOptions::default(), begin, update, draw, end)?;
    *cancel.borrow_mut() = Some(Box::new(cancel_fn));


    Ok(())
}


fn get_remaining(elapsed:u64) -> u64 {
    if MAX > elapsed {
        MAX - elapsed
    } else {
        0
    }
}
