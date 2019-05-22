//use awsm::*;
use wasm_bindgen::prelude::*;
//use web_sys::{console};
use crate::menu::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;

// Called by our JS entry point to run the example.
pub fn start(window:web_sys::Window, document:web_sys::Document) -> Result<(), JsValue> {
    let window = Rc::new(window);

    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();

    let pathname = window.location().pathname()?;

    let pathname = pathname.as_str();

    match pathname {
        "/" => {
            let menu = menu::build_menu(Rc::clone(&window), &document)?;
            body.append_child(&menu)?;
        },

        "/foo" => {
        },

        _ => {

            let text = format!("unknown route: {}", &pathname);
            let item: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
            item.set_text_content(Some(&text));

            body.append_child(&item)?;
        }
    }


    Ok(())
}

