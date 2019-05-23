use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Document, Node, HtmlElement};

pub fn start(window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {
    let element = create_element(&document, &body)?;

    element.set_text_content(Some("TODO: Update clock with current time"));
    
    Ok(())
}

fn create_element(document: &Document, body: &HtmlElement) -> Result<HtmlElement, JsValue> {
    let text = "Clock here!!";

    let item: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_text_content(Some(&text));

    body.append_child(&item)?;

    Ok(item)

}
