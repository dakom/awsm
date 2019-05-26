use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Node, Element, HtmlElement, HtmlHyperlinkElementUtils};

pub fn build_menu(document:&Document) -> Result<web_sys::Node, JsValue> {
    let container: Node = document.create_element("div")?.into();

    append_menu(&container, &document, "Tick", vec![
       ("clock", "Clock"),
    ])?;

    append_menu(&container, &document, "WebGl", vec![
       ("webgl-simple", "Simple"),
       ("webgl-texture", "Texture"),
       ("webgl-instancing", "Instancing"),
    ])?;

    Ok(container)
}

fn append_menu (container:&Node, document:&Document, label:&str, menu_labels:Vec<(&str, &str)>) -> Result<(), JsValue> {

    let menu: Element = document.create_element("div")?.into();
    menu.set_class_name("menu");

    let header: Element = document.create_element("div")?.into();
    header.set_class_name("menu-header");
    header.set_text_content(Some(&label));

    let menu_list: Element = document.create_element("div")?.into();
    menu_list.set_class_name("menu-list");

    for link_info in menu_labels.into_iter() {
        let item = create_menu_item(&link_info, document)?;
        menu_list.append_child(&item)?;
    }


    menu.append_child(&header)?;
    menu.append_child(&menu_list)?;

    container.append_child(&menu)?;
    Ok(())

}

fn create_menu_item(link_info:&(&str, &str), document:&Document) -> Result<HtmlElement, JsValue> {

    let (href, label) = link_info;

    let item: HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_text_content(Some(&label));

    wrap_link(&href, item, &document)
}

fn wrap_link(href:&str, contents:HtmlElement, document:&Document) -> Result<HtmlElement, JsValue> {
    let anchor: HtmlElement = document.create_element("a")?.dyn_into()?;

    anchor.append_child(&contents)?;
    
    let anchor = anchor.unchecked_into::<HtmlHyperlinkElementUtils>();
    anchor.set_href(&href);

    let anchor = anchor.unchecked_into::<HtmlElement>();

    Ok(anchor)
}
