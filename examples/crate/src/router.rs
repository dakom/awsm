use wasm_bindgen::prelude::*;
use super::menu;
use super::scenes::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, HtmlHyperlinkElementUtils};
use cfg_if::cfg_if;
// Called by our JS entry point to run the example.
pub fn start_router(window:web_sys::Window, document:web_sys::Document) -> Result<(), JsValue> {
    let body = document.body().expect("should have body");

    let pathname = window.location().pathname()?;

    let pathname = get_root(pathname.as_str());


    if let Some(menu) = menu::MENU_LOOKUP.get(pathname) {
        let links: Element = document.create_element("div")?.into();
        links.set_class_name("demo-links");
        body.append_child(&links)?;

        let home_link = create_home_link(&document)?;
        links.append_child(&home_link)?;

        let href = format!("https://github.com/dakom/awsm/tree/master/examples/crate/src/scenes/{}", menu.source);
        let source_link = create_source_link(&href, &document)?;
        links.append_child(&source_link)?;
    }

    match pathname {
        "" => {
            let menu = menu::build_menu(&document)?;
            body.append_child(&menu)?;
            Ok(())
        },

        "clock" => {
            clock::start(window, document, body)
        }

        "loaders-image" => {
            loaders::image::start(window, document, body)
        }
        "webgl-simple" => {
            webgl::simple::start(window, document, body)
        }
        _ => {

            let text = format!("unknown route: {}", &pathname);
            let item: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
            item.set_text_content(Some(&text));

            body.append_child(&item)?;
            Ok(())
        }
    }
}


// enable logging only during debug builds 
cfg_if! {
    if #[cfg(debug_assertions)] {
        fn get_home_href() -> &'static str {
            "/"
        }
    } else {
        fn get_home_href() -> &'static str {
            "/awsm/"
        }
    }
}
fn create_home_link(document:&Document) -> Result<HtmlElement, JsValue> {
    let anchor: HtmlElement = document.create_element("a")?.dyn_into()?;

    let contents: Element = document.create_element("div")?.into();
    contents.set_class_name("home button");
    contents.set_text_content(Some("Home"));
    anchor.append_child(&contents)?;
    
    let anchor = anchor.unchecked_into::<HtmlHyperlinkElementUtils>();
    anchor.set_href(get_home_href());

    let anchor = anchor.unchecked_into::<HtmlElement>();

    Ok(anchor)
}
fn create_source_link(href:&str, document:&Document) -> Result<HtmlElement, JsValue> {
    let anchor: HtmlElement = document.create_element("a")?.dyn_into()?;

    let contents: Element = document.create_element("div")?.into();
    contents.set_class_name("source button");
    contents.set_text_content(Some("View Source"));
    anchor.append_child(&contents)?;
    
    let anchor = anchor.unchecked_into::<HtmlHyperlinkElementUtils>();
    anchor.set_href(&href);

    let anchor = anchor.unchecked_into::<HtmlElement>();

    Ok(anchor)
}

fn get_root(input:&str) -> &str {
    //account for github / relative path
    let stripped = match input.find("awsm/") {
        Some(len) => {
            input.split_at(len + 4).1
        },

        None => {
            match input.find("/") {
                Some(len) => input.split_at(len + 1).1,
                None => input
            }
        }
    };

    stripped.trim_matches('/')

}

#[test]
fn routes() {
    assert_eq!(get_root("/foo"), "foo");
    assert_eq!(get_root("/"), "");
    assert_eq!(get_root("/awsm/foo"), "foo");
    assert_eq!(get_root("/awsm/"), "");
}
