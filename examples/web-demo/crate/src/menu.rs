use lazy_static::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlAnchorElement, HtmlElement, Node};

pub struct Menu<'a> {
    pub label: &'a str,
    pub source: &'a str,
}

lazy_static! {
    pub static ref MENU_LOOKUP: HashMap<&'static str, Menu<'static>> = {
        let mut m = HashMap::new();
        //Tick
        m.insert("tick-raf", Menu {label: "rAF", source: "tick/raf.rs"});
        m.insert("tick-mainloop", Menu {label: "Main Loop", source: "tick/mainloop.rs"});
        //Loaders
        m.insert("loaders-image", Menu {label: "Image", source: "loaders/image.rs"});
        m.insert("loaders-image-data", Menu {label: "Image Data", source: "loaders/image_data.rs"});
        m.insert("loaders-text", Menu {label: "Text", source: "loaders/text.rs"});
        //Input
        m.insert("input-pointer-lock", Menu {label: "Pointer Lock", source: "input/pointer_lock.rs"});
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
        //Audio
        m.insert("audio-player", Menu {label: "Player", source: "audio/player.rs"});

        //return
        m
    };
}

pub fn build_menu(document: &Document) -> Result<web_sys::Node, JsValue> {
    let container: Node = document.create_element("div")?.into();

    append_home_button(&container, &document)?;

    append_menu(
        &container,
        &document,
        "Tick Loop",
        &vec!["tick-raf", "tick-mainloop"],
        None,
    )?;

    append_menu(
        &container,
        &document,
        "Loaders",
        &vec!["loaders-image", "loaders-image-data", "loaders-text"],
        None,
    )?;

    append_menu(
        &container,
        &document,
        "Input",
        &vec!["input-pointer-lock"],
        None,
    )?;

    let webgl_menu_common = vec![
        "webgl-simple",
        "webgl-texture",
        "webgl-multi-texture",
        "webgl-blending",
        "webgl-elements",
        "webgl-instancing",
        "webgl-vaos",
        "webgl-texture_cube",
    ];

    let webgl_menu_1 = webgl_menu_common.clone();
    let mut webgl_menu_2 = webgl_menu_common.clone();
    webgl_menu_2.extend(vec!["webgl-ubos", "webgl-texture_3d"]);

    append_menu(
        &container,
        &document,
        "WebGl 1",
        &webgl_menu_1,
        Some("?webgl=1"),
    )?;
    append_menu(
        &container,
        &document,
        "WebGl 2",
        &webgl_menu_2,
        Some("?webgl=2"),
    )?;

    append_menu(&container, &document, "Audio", &vec!["audio-player"], None)?;

    Ok(container)
}

fn append_home_button(container: &Node, document: &Document) -> Result<(), JsValue> {
    let item: HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_class_name("button");
    item.set_text_content(Some("Home"));

    let link = wrap_link("/", item, &document)?;

    let item: HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_class_name("home-button");

    item.append_child(&link)?;
    container.append_child(&item)?;

    Ok(())
}
fn append_menu(
    container: &Node,
    document: &Document,
    label: &str,
    menu_routes: &[&str],
    menu_suffix: Option<&str>,
) -> Result<(), JsValue> {
    let menu_element: Element = document.create_element("div")?.into();
    menu_element.set_class_name("menu");

    let header: Element = document.create_element("div")?.into();
    header.set_class_name("menu-header");
    header.set_text_content(Some(&label));

    let menu_list: Element = document.create_element("div")?.into();
    menu_list.set_class_name("menu-list");

    for menu_route in menu_routes.into_iter() {
        if let Some(menu) = MENU_LOOKUP.get(menu_route) {
            let href = match menu_suffix {
                None => format!("{}", menu_route),
                Some(suffix) => format!("{}{}", menu_route, suffix),
            };

            let item = create_menu_item(&href, &menu, document)?;
            menu_list.append_child(&item)?;
        }
    }

    menu_element.append_child(&header)?;
    menu_element.append_child(&menu_list)?;

    container.append_child(&menu_element)?;
    Ok(())
}

fn create_menu_item(href: &str, menu: &Menu, document: &Document) -> Result<HtmlElement, JsValue> {
    let item: HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_class_name("button");
    item.set_text_content(Some(&menu.label));

    wrap_link(&href, item, &document)
}

fn wrap_link(
    href: &str,
    contents: HtmlElement,
    document: &Document,
) -> Result<HtmlElement, JsValue> {
    let anchor: HtmlElement = document.create_element("a")?.dyn_into()?;

    anchor.append_child(&contents)?;

    let anchor = anchor.unchecked_into::<HtmlAnchorElement>();
    anchor.set_href(&href);

    let anchor = anchor.unchecked_into::<HtmlElement>();

    Ok(anchor)
}
