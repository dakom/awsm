use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console};

//TODO - call closure with label of scene
pub fn build_menu(document:&web_sys::Document) -> Result<web_sys::Node, JsValue> {
    let container: web_sys::Node = document.create_element("div")?.into();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            //context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            //pressed.set(true);
            console::log_1(&JsValue::from_str("got click!"))
        }) as Box<dyn FnMut(_)>);


    append_menu(&closure, &container, &document, "Tick", &vec![
       "Clock",
    ])?;

    append_menu(&closure, &container, &document, "WebGl", &vec![
       "Quad",
       "Quad - Texture",
       "Quad - Instancing",
    ])?;

    closure.forget();

    Ok(container)
}

fn append_menu(on_click: &Closure<dyn FnMut(web_sys::MouseEvent)>, container:&web_sys::Node, document:&web_sys::Document, label:&str, menu_labels:&Vec<&str>) -> Result<(), JsValue> {

    let menu: web_sys::Element = document.create_element("div")?.into();
    menu.set_class_name("menu");

    let header: web_sys::Element = document.create_element("div")?.into();
    header.set_class_name("menu-header");
    header.set_text_content(Some(&label));

    let menu_list: web_sys::Element = document.create_element("div")?.into();
    menu_list.set_class_name("menu-list");

    //let menu: web_sys::Node = menu.into();

    for label in menu_labels.iter() {
        let item = create_menu_item(*label, document)?;
        menu_list.append_child(&item);

        let on_click = on_click.as_ref().unchecked_ref();
        item.set_onclick(Some(&on_click));
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