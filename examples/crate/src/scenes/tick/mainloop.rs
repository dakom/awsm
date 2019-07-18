use awsm::tick;
use awsm::tick::{MainLoop};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Element, Document, HtmlElement};
use std::rc::Rc;
use std::cell::RefCell;

pub fn start(_window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {

    let root: Element = document.create_element("div")?.into();
    root.set_class_name("tick");
    body.append_child(&root)?;

    let header: web_sys::HtmlElement = document.create_element("h1")?.dyn_into()?;
    header.set_text_content(Some("Waiting for first tick"));
    root.append_child(&header)?;

    let info: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
    root.append_child(&info)?;


    let begin = move |time, delta| {
    };

    let update = move |delta| {
    };

    let draw = move |interpolation| {
    };
    let end = move |fps, abort| {
    };
    let mut main_loop = MainLoop::new(begin, update, draw, end);


    main_loop.start();
    /*
    let cancel_fn = tick::start_main_loop({
        move |time_stamp| {
            let Timestamp {time, delta, elapsed} = time_stamp;

            let elapsed = (elapsed / 1000.0).round() as u64;

            let my_str = format!("{} seconds left till stopping the ticker!", get_remaining(elapsed));
            header.set_text_content(Some(&my_str.as_str()));
            let my_str = format!("tick: {} delta: {}", time, delta);
            info.set_text_content(Some(&my_str.as_str()));

            if elapsed > MAX { 
                if let Some(cb) = cancel.borrow_mut().take() {
                    cb();
                }
                header.set_text_content(Some("ticker stopped!"));
            }
        }
    })?;
  
    std::mem::forget(Box::new(cancel_fn));
    */


    Ok(())
}
