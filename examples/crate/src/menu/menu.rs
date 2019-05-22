use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
//use web_sys::{console};
use gloo_events::EventListener;
use std::rc::Rc;
pub fn build_menu(window:Rc<web_sys::Window>, document:&web_sys::Document) -> Result<web_sys::Node, JsValue> {
    let container: web_sys::Node = document.create_element("div")?.into();

    //Hmmmmm
    let on_click = Rc::new(move |label:&str| {

        let pathname = format!("/{}", &label);
        window.location().set_pathname(&pathname);
    });

    append_menu(Rc::clone(&on_click), &container, &document, "Tick", vec![
       "Clock",
    ])?;

    append_menu(Rc::clone(&on_click), &container, &document, "WebGl", vec![
       "Quad",
       "Quad - Texture",
       "Quad - Instancing",
    ])?;

    //closure.forget();

    Ok(container)
}

fn append_menu <F> (on_click: Rc<F>, container:&web_sys::Node, document:&web_sys::Document, label:&'static str, menu_labels:Vec<&'static str>) -> Result<(), JsValue> 
where F: (Fn(&str) -> ()) + 'static,
{

    let menu: web_sys::Element = document.create_element("div")?.into();
    menu.set_class_name("menu");

    let header: web_sys::Element = document.create_element("div")?.into();
    header.set_class_name("menu-header");
    header.set_text_content(Some(&label));

    let menu_list: web_sys::Element = document.create_element("div")?.into();
    menu_list.set_class_name("menu-list");

    for label in menu_labels.into_iter() {
        let item = create_menu_item(&label, document)?;
        menu_list.append_child(&item);

        //need to clone this since each closure will take ownership
        let on_click = Rc::clone(&on_click);

        EventListener::new(&item, "click", move |_e| {
            on_click(&label);
        }).forget();
    }


    menu.append_child(&header);
    menu.append_child(&menu_list);

    container.append_child(&menu);
    Ok(())

}

fn create_menu_item(label:&str, document:&web_sys::Document) -> Result<web_sys::HtmlElement, JsValue> {

    let item: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_text_content(Some(&label));

    Ok(item)

}