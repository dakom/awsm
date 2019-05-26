use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document, HtmlElement};

pub fn start(_window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {
    Ok(())
}


