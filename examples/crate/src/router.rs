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
        },

        "loaders-image" => {
            loaders::image::start(window, document, body)
        },


        "webgl-simple" => {
            webgl::simple::start(window, document, body)
        },

        "webgl-texture" => {
            webgl::texture::start(window, document, body)
        },

        "webgl-multi-texture" => {
            webgl::multi_texture::start(window, document, body)
        },

        "webgl-blending" => {
            webgl::blending::start(window, document, body)
        },

        "webgl-elements" => {
            webgl::elements::start(window, document, body)
        },

        "webgl-instancing" => {
            webgl::instancing::start(window, document, body)
        },

        "webgl-vaos" => {
            webgl::vaos::start(window, document, body)
        },
        "webgl-ubos" => {
            start_additional_menu(&pathname, window, document, body)
        },
        _ => {
            unknown_route(&pathname, window, document, body)
        }
    }
}


cfg_if! {
    if #[cfg(feature = "webgl_2")] {
        fn start_additional_menu(pathname:&str, window:web_sys::Window, document:web_sys::Document, body:web_sys::HtmlElement) -> Result<(), JsValue> {
            match pathname {
                "webgl-ubos" => webgl::ubos::start(window, document, body),
                _ => unknown_route(&pathname, window, document, body)
            }
        }
    } else {
        fn start_additional_menu(pathname:&str, window:web_sys::Window, document:web_sys::Document, body:web_sys::HtmlElement) -> Result<(), JsValue> {
            match pathname {
                _ => unknown_route(&pathname, window, document, body)
            }
        }
    }
}

fn unknown_route(pathname:&str, _window:web_sys::Window, document:web_sys::Document, body:web_sys::HtmlElement) -> Result<(), JsValue> {
    let text = format!("unknown route: {}", &pathname);
    let item: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_text_content(Some(&text));

    body.append_child(&item)?;

    Ok(())
}
//Production deploys separate webgl1 vs webgl2 builds into their own directory
//Dev is one at a time in the root
cfg_if! {
    if #[cfg(debug_assertions)] {
        pub fn get_home_href() -> &'static str {
            "/"
        }
    } else if #[cfg(feature = "webgl_1")] {
        pub fn get_home_href() -> &'static str {
            "/webgl1/"
        }
    } else if #[cfg(feature = "webgl_2")] {
        pub fn get_home_href() -> &'static str {
            "/webgl2/"
        }
    } else {
        pub fn get_home_href() -> &'static str {
            "/"
        }
    }
}

//Just basic stripping of urls to account for the difference in dev vs production
//giving it unicode might go wonky
fn get_root(input:&str) -> &str {
    let strip_matched = |prefix:&str| -> Option<&str> {
        input
            .find(prefix)
            .map(|len| input.split_at(len + prefix.len()-1).1)
    };

    let stripped = 
        strip_matched("webgl1/")
        .or(strip_matched("webgl2/"))
        .or(Some(input))
        .unwrap();

    stripped.trim_matches('/')

}

pub fn get_static_href(path:&str) -> String {
    format!("/static/{}", path)
}

fn create_home_link(document:&Document) -> Result<HtmlElement, JsValue> {
    let anchor: HtmlElement = document.create_element("a")?.dyn_into()?;

    let contents: Element = document.create_element("div")?.into();
    contents.set_class_name("home button");
    contents.set_text_content(Some("Menu")); //It's not the ultimate home on deploys which has webgl1/2 menu on the root
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

#[test]
fn routes() {
    //get_root
    assert_eq!(get_root("/foo"), "foo");
    assert_eq!(get_root("/foo/bar/"), "foo/bar");
    assert_eq!(get_root("/"), "");
    assert_eq!(get_root("/webgl1/foo"), "foo");
    assert_eq!(get_root("/webgl1/foo/bar/"), "foo/bar");
    assert_eq!(get_root("/webgl1/"), "");
    assert_eq!(get_root("/webgl2/foo"), "foo");
    assert_eq!(get_root("/webgl2/foo/bar/"), "foo/bar");
    assert_eq!(get_root("/webgl2/"), "");
    assert_eq!(get_root("/static/file.jpg"), "static/file.jpg");
    //get_static
    assert_eq!(get_static_href("images/smiley.svg"), "/static/images/smiley.svg");
}
