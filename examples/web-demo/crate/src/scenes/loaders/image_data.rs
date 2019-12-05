use crate::router::get_static_href;
use awsm_web::loaders::fetch;
use gloo_events::EventListener;
use log::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Document, Element, HtmlElement, Window};

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
        move |_e: &web_sys::Event| {
            let future = async move {
                let href = get_static_href("photo.jpg");
                info!("loading image! {}", href);
                let data = fetch::vec_u8(&href).await?;
                let img = fetch::image_u8(&data, "image/jpg").await?;
                info!("loaded!!! {}", img.src());
                root.append_child(&img).map(|_| JsValue::null())
            };

            //we don't handle errors here because they are exceptions
            //hope you're running in an environment where uncaught rejects/exceptions are reported!
            future_to_promise(future);

            root_copy.remove_child(&button).unwrap();
        }
    };

    //for demo purposes - fine to forget
    EventListener::once(&button, "click", my_cb).forget();

    Ok(())
}

fn create_button(document: &Document, root: &Element) -> Result<HtmlElement, JsValue> {
    let item: HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_class_name("button demo-button");
    item.set_text_content(Some("load image"));
    root.append_child(&item)?;
    Ok(item)
}
