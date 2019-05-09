extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

//FOLLOWUP: https://github.com/rustwasm/wasm-bindgen/issues/939#issuecomment-451751667

pub fn quick_check() -> Result<(), js_sys::Error> {

    let xhr = XmlHttpRequest::new()?;

    let cb = |evt:JsValue| {
        //evt is a JsValue... I think... we want to get evt.target.status and evt.target.response
        console::log_1(&evt);
    };

    let cb = Closure::wrap(Box::new(cb) as Box<Fn(JsValue) -> ()>);

    xhr.set_onreadystatechange(Some(cb.as_ref().unchecked_ref()));

    xhr.open_with_async(&"GET", &"https://jsonplaceholder.typicode.com/todos/1", true)?;
    xhr.send()?;

    //TODO - manage this better... should return in a future somehow?
    cb.forget();

    Ok(())
}
/*
pub fn fetch_image(url:&str) -> Result<HtmlImageElement, JsValue> {
    let img = HtmlImageElement::new()?;

    let has_same_origin = same_origin(url)?;

    if !has_same_origin {
        img.set_cross_origin(Some(&"anonymous"));
    }

    let cb_onload = |evt:JsValue| {
        //evt is a JsValue... I think... we want to get evt.target
        console::log_1(&evt);
    };

    let cb_onload = Closure::wrap(Box::new(cb_onload) as Box<Fn(JsValue) -> ()>);

    img.set_onload(Some(cb_onload.as_ref().unchecked_ref()));


    let cb_onerror = |evt:JsValue| {
        //evt is a JsValue... I think... we want to get evt.target
        console::error_1(&evt);
    };

    let cb_onerror = Closure::wrap(Box::new(cb_onerror) as Box<Fn(JsValue) -> ()>);

    img.set_onerror(Some(cb_onerror.as_ref().unchecked_ref()));

    //TODO - manage this better... should return in a future somehow?
    cb_onload.forget();
    cb_onerror.forget();

    img.set_src(url);
    Ok(img)
}

*/
