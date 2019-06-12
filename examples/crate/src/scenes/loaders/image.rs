use awsm::loaders::{image};
use awsm::errors::{Error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document, Element, HtmlElement};
use gloo_events::{EventListener};
use log::{info};
use crate::router::{get_static_href};
use wasm_bindgen_futures::futures_0_3::{future_to_promise};

pub fn start(_window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {

    let root: Element = document.create_element("div")?.into();
    root.set_class_name("loaders");
    body.append_child(&root)?;

    let button = create_button(&document, &root)?;
    
    let my_cb = {
        //button will be owned by the closure
        let button = button.clone();
        //root needs to be owned by both closure and async
        let root_copy = root.clone();
        move |_e:&web_sys::Event| {
            let future = async move {
                let href = get_static_href("smiley.svg");
                info!("loading image! {}", href);
                match image::fetch_image(href).await {
                    Ok(img) => {
                        info!("loaded!!! {}", img.src());
                        root.append_child(&img)
                            .map(|_| JsValue::null())
                    }
                    Err(err) => {
                        info!("error!");
                        Err(err.into())
                    }
                }
            };

            //we don't handle errors here because they are exceptions
            //hope you're running in an environment where uncaught rejects/exceptions are reported!
            future_to_promise(future);


            root_copy.remove_child(&button).unwrap();
        }
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


