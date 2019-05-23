use awsm::tick;
use awsm::tick::{Timestamp};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Element, Document, HtmlElement};
use std::cell::Cell;
use std::rc::Rc;

const MAX:u64 = 10;

pub fn start(_window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {

    let root: Element = document.create_element("div")?.into();
    root.set_class_name("tick");
    body.append_child(&root)?;

    let header: web_sys::HtmlElement = document.create_element("h1")?.dyn_into()?;
    header.set_text_content(Some("Waiting for first tick"));
    root.append_child(&header)?;

    let info: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
    root.append_child(&info)?;

    //this has 2 owners: 
    //1. the start_loop internal rAF callback
    //2. the business-logic callback we pass in
    //
    //Therefore it needs to be Rc 
    //
    //It also needs to be mutable, e.g. interior mutability
    //Since it's Copy (boolean), Cell is enough
    let keep_alive = Rc::new(Cell::new(true));

    tick::start_loop(Rc::clone(&keep_alive), move |time_stamp| {
        let Timestamp {time, delta, elapsed} = time_stamp;

        let elapsed = (elapsed / 1000.0).round() as u64;

        let my_str = format!("{} seconds left till stopping the ticker!", get_remaining(elapsed));
        header.set_text_content(Some(&my_str.as_str()));
        let my_str = format!("tick: {} delta: {}", time, delta);
        info.set_text_content(Some(&my_str.as_str()));

        if elapsed > MAX { 
            keep_alive.set(false);
            header.set_text_content(Some("ticker stopped!"));
        }
    })?;
    
    Ok(())
}

fn get_remaining(elapsed:u64) -> u64 {
    if MAX > elapsed {
        MAX - elapsed
    } else {
        0
    }
}
