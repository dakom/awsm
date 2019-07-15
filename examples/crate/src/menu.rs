use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Node, Element, HtmlElement, HtmlAnchorElement};
use lazy_static::*;
use std::collections::HashMap;
use cfg_if::cfg_if;

pub struct Menu <'a> {
    pub label: &'a str,
    pub source: &'a str,
}

lazy_static! {
    pub static ref MENU_LOOKUP: HashMap<&'static str, Menu<'static>> = {
        let mut m = HashMap::new();
        //Tick
        m.insert("clock", Menu {label: "Clock", source: "clock/clock.rs"});
        //Loaders
        m.insert("loaders-image", Menu {label: "Image", source: "loaders/image.rs"});
        m.insert("loaders-text", Menu {label: "Text", source: "loaders/text.rs"});
        //WebGl
        m.insert("webgl-simple", Menu {label: "Simple", source: "webgl/simple/simple.rs"});
        m.insert("webgl-texture", Menu {label: "Texture", source: "webgl/texture/texture.rs"});
        m.insert("webgl-multi-texture", Menu {label: "Multi-Texture", source: "webgl/multi_texture/multi_texture.rs"});
        m.insert("webgl-blending", Menu {label: "Blending", source: "webgl/blending/blending.rs"});
        m.insert("webgl-elements", Menu {label: "Elements", source: "webgl/elements/elements.rs"});
        m.insert("webgl-instancing", Menu {label: "Instancing", source: "webgl/instancing/instancing.rs"});
        m.insert("webgl-vaos", Menu {label: "Vertex Arrays", source: "webgl/vaos/vaos.rs"});
        m.insert("webgl-ubos", Menu {label: "Uniform Buffers", source: "webgl/ubos/ubos.rs"});
        m.insert("webgl-texture_3d", Menu {label: "Texture 3D", source: "webgl/texture_3d/texture_3d.rs"});
        m.insert("webgl-texture_cube", Menu {label: "Texture Cubemap", source: "webgl/texture_cube/texture_cube.rs"});

        m
    };
}

#[cfg(feature = "webgl_1")]
fn get_webgl_title() -> &'static str {
    "WebGl (version 1)"
}
#[cfg(feature = "webgl_2")]
fn get_webgl_title() -> &'static str {
    "WebGl (version 2)"
}

pub fn build_menu(document:&Document) -> Result<web_sys::Node, JsValue> {
    let container: Node = document.create_element("div")?.into();

    append_home_button(&container, &document)?;

    append_menu(&container, &document, "Ticker", vec![
      "clock" 
    ])?;

    append_menu(&container, &document, "Loaders", vec![
        "loaders-image",
        "loaders-text",
    ])?;

    let mut webgl_menu = vec![
        "webgl-simple",
        "webgl-texture",
        "webgl-multi-texture",
        "webgl-blending",
        "webgl-elements",
        "webgl-instancing",
        "webgl-vaos",
        "webgl-texture_cube",
    ];

    cfg_if! {
        if #[cfg(feature = "webgl_2")] {
            fn concat_more_menus(menus:&mut Vec<&str>) {
                menus.push("webgl-ubos");
                menus.push("webgl-texture_3d");
            }
        } else {
            fn concat_more_menus(menus:&mut Vec<&str>) {
            }
        }
    }

    concat_more_menus(&mut webgl_menu);

    append_menu(&container, &document, get_webgl_title(), webgl_menu)?;

    Ok(container)
}

fn append_home_button (container:&Node, document:&Document) -> Result<(), JsValue> {

    let item:HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_class_name("button");
    item.set_text_content(Some("Home"));

    let link = wrap_link("/", item, &document)?;

    let item:HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_class_name("home-button");

    item.append_child(&link)?;
    container.append_child(&item)?;
    
    Ok(())

}
fn append_menu (container:&Node, document:&Document, label:&str, menu_routes:Vec<&str>) -> Result<(), JsValue> {

    let menu_element: Element = document.create_element("div")?.into();
    menu_element.set_class_name("menu");

    let header: Element = document.create_element("div")?.into();
    header.set_class_name("menu-header");
    header.set_text_content(Some(&label));

    let menu_list: Element = document.create_element("div")?.into();
    menu_list.set_class_name("menu-list");

    for menu_route in menu_routes.into_iter() {
        if let Some(menu) = MENU_LOOKUP.get(menu_route) {
            let item = create_menu_item(&menu_route, &menu, document)?;
            menu_list.append_child(&item)?;
        }
    }

    menu_element.append_child(&header)?;
    menu_element.append_child(&menu_list)?;

    container.append_child(&menu_element)?;
    Ok(())

}

fn create_menu_item(href:&str, menu:&Menu, document:&Document) -> Result<HtmlElement, JsValue> {

    let item: HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_class_name("button");
    item.set_text_content(Some(&menu.label));

    wrap_link(&href, item, &document)
}

fn wrap_link(href:&str, contents:HtmlElement, document:&Document) -> Result<HtmlElement, JsValue> {
    let anchor: HtmlElement = document.create_element("a")?.dyn_into()?;

    anchor.append_child(&contents)?;
    
    let anchor = anchor.unchecked_into::<HtmlAnchorElement>();
    anchor.set_href(&href);

    let anchor = anchor.unchecked_into::<HtmlElement>();

    Ok(anchor)
}
