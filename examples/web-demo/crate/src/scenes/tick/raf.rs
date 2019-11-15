use awsm_web::tick::{Timestamp, TimestampLoop};
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

    let info: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
    root.append_child(&info)?;

    //Closure needs to take ownership since it occurs past the JS boundry and is 'static
    //but we need to assign the value of cancel from outside the closure
    let timestamp_loop: Rc<RefCell<Option<TimestampLoop>>> = Rc::new(RefCell::new(None));

    let _timestamp_loop = TimestampLoop::start({
        let timestamp_loop = Rc::clone(&timestamp_loop);
        move |time_stamp| {
            let Timestamp {
                time,
                delta,
                elapsed,
            } = time_stamp;

            let elapsed = (elapsed / 1000.0).round() as u64;

            let my_str = format!(
                "{} seconds left till stopping the ticker!",
                get_remaining(elapsed)
            );
            header.set_text_content(Some(&my_str.as_str()));
            let my_str = format!("tick: {} delta: {}", time, delta);
            info.set_text_content(Some(&my_str.as_str()));

            if elapsed > MAX {
                timestamp_loop.borrow_mut().take();
                header.set_text_content(Some("ticker stopped!"));
            }
        }
    })?;

    *timestamp_loop.borrow_mut() = Some(_timestamp_loop);

    Ok(())
}

fn get_remaining(elapsed: u64) -> u64 {
    if MAX > elapsed {
        MAX - elapsed
    } else {
        0
    }
}
