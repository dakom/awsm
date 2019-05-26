use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document, Element, HtmlElement};
use gloo_events::{EventListener};
use log::{info};
use std::rc::Rc;
use crate::router::{get_static_href};

pub fn start(_window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {

    let root: Element = document.create_element("div")?.into();
    root.set_class_name("loaders");
    body.append_child(&root)?;

    let button = create_button(&document, &root)?;
    let button = Rc::new(button);
    {
        EventListener::once(&button, "click", {
            let button = Rc::clone(&button);
            move |_| {
                let href = get_static_href("smiley.svg");
                info!("loading image! {}", href);
                root.remove_child(&button);
            }
        }).forget();
    }

    Ok(())
}


fn create_button(document:&Document, root:&Element) -> Result<HtmlElement, JsValue> {

    let button: HtmlElement = document.create_element("button")?.dyn_into()?;
    button.set_text_content(Some("load image"));

    root.append_child(&button)?;

    Ok(button)
}


