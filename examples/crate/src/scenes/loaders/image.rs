use awsm::loaders::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document, Element, HtmlElement};
use futures::future::Future;
use gloo_events::{EventListener};
use log::{info};
use std::rc::Rc;
use crate::router::{get_static_href};
use wasm_bindgen_futures::{future_to_promise};

pub fn start(_window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {

    let root: Element = document.create_element("div")?.into();
    root.set_class_name("loaders");
    body.append_child(&root)?;

    let button = create_button(&document, &root)?;
    let button = Rc::new(button);
    
    let button_ref = Rc::clone(&button);
    let my_cb = move |_e:&web_sys::Event| {
        let href = get_static_href("smiley.svg");
        info!("loading image!{}", href);
        let future = image::fetch_image(href)
            .and_then({
                let root = root.clone();
                move |img| {
                    info!("loaded!!!");
                    futures::done(root.append_child(&img))
                    //for seeing what a fail looks like:
                    //futures::failed::<JsValue, JsValue>(JsValue::from_str("uhuh!!"))
                }
            });

        //the future resolves with a Node, but future_to_promise expects JsValue
        //we don't handle errors here because they are exceptions
        //hope you're running in an environment where uncaught rejects/exceptions are reported!
        future_to_promise(future.map(|_| JsValue::null()));

        root.remove_child(&button_ref).unwrap();
    };

    //for demo purposes - fine to forget
    EventListener::once(&button, "click",my_cb).forget();

    Ok(())
}


fn create_button(document:&Document, root:&Element) -> Result<HtmlElement, JsValue> {

    let button: HtmlElement = document.create_element("button")?.dyn_into()?;
    button.set_text_content(Some("load image"));

    root.append_child(&button)?;

    Ok(button)
}


