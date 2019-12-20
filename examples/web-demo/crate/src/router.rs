use super::menu;
use super::scenes::*;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlAnchorElement, HtmlElement};
// Called by our JS entry point to run the example.
pub fn start_router(window: web_sys::Window, document: web_sys::Document) -> Result<(), JsValue> {
    let body = document.body().expect("should have body");

    let pathname = window.location().pathname()?;

    let pathname = get_root(pathname.as_str());

    if let Some(menu) = menu::MENU_LOOKUP.get(pathname) {
        let links: Element = document.create_element("div")?.into();
        links.set_class_name("demo-links");
        body.append_child(&links)?;

        let home_link = create_home_link(&document)?;
        links.append_child(&home_link)?;

        let href = format!(
            "https://github.com/dakom/awsm/tree/master/examples/web-demo/crate/src/scenes/{}",
            menu.source
        );
        let source_link = create_source_link(&href, &document)?;
        links.append_child(&source_link)?;
    }

    let _search = window.location().search()?;

    let webgl_version = if pathname.contains("webgl-") {
        let query_value = get_query_value(&window, "webgl").and_then(|s| {
            if s == "1" {
                Some(webgl::common::WebGlVersion::One)
            } else if s == "2" {
                Some(webgl::common::WebGlVersion::Two)
            } else {
                None
            }
        });

        match query_value {
            Some(value) => Ok(value),
            None => Err(JsValue::from_str("Unable to get webgl version")),
        }
    } else {
        //not passed along, but just in case..
        Ok(webgl::common::WebGlVersion::Two)
    }?;

    match pathname {
        "" => {
            let menu = menu::build_menu(&document)?;
            body.append_child(&menu)?;
            Ok(())
        }

        "tick-raf" => tick::raf::start(window, document, body),
        "tick-mainloop" => tick::mainloop::start(window, document, body),

        "loaders-image" => loaders::image::start(window, document, body),
        "loaders-image-data" => loaders::image_data::start(window, document, body),
        "loaders-text" => loaders::text::start(window, document, body),

        "input-pointer-lock" => input::pointer_lock::start(window, document, body),

        "webgl-simple" => webgl::simple::start(window, document, body, webgl_version),
        "webgl-texture" => webgl::texture::start(window, document, body, webgl_version),
        "webgl-multi-texture" => webgl::multi_texture::start(window, document, body, webgl_version),
        "webgl-blending" => webgl::blending::start(window, document, body, webgl_version),
        "webgl-elements" => webgl::elements::start(window, document, body, webgl_version),
        "webgl-instancing" => webgl::instancing::start(window, document, body, webgl_version),
        "webgl-vaos" => webgl::vaos::start(window, document, body, webgl_version),
        "webgl-texture_cube" => webgl::texture_cube::start(window, document, body, webgl_version),
        "webgl-ubos" => webgl::ubos::start(window, document, body),
        "webgl-texture_3d" => webgl::texture_3d::start(window, document, body),
        "audio-player" => audio::player::start(window, document, body),
        _ => unknown_route(&pathname, window, document, body),
    }
}

fn get_query_value(window: &web_sys::Window, key: &str) -> Option<String> {
    let search = match window.location().search() {
        Ok(value) => value,
        Err(_) => "".to_owned(),
    };

    let needle = format!("{}=", key);

    match search.find(&needle) {
        None => None,
        Some(offset_start) => {
            let offset_start = offset_start + key.len() + 1;
            let value = &search[offset_start..];
            let value = value.trim();
            match value
                .chars()
                .position(|c| !c.is_alphanumeric() && c != '-' && c != '_')
            {
                None => Some(value.to_owned()),
                Some(offset_end) => {
                    let value = &value[..offset_end];
                    Some(value.to_owned())
                }
            }
        }
    }
}

fn unknown_route(
    pathname: &str,
    _window: web_sys::Window,
    document: web_sys::Document,
    body: web_sys::HtmlElement,
) -> Result<(), JsValue> {
    let text = format!("unknown route: {}", &pathname);
    let item: web_sys::HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_text_content(Some(&text));

    body.append_child(&item)?;

    Ok(())
}


fn create_home_link(document: &Document) -> Result<HtmlElement, JsValue> {
    let anchor: HtmlElement = document.create_element("a")?.dyn_into()?;

    let contents: Element = document.create_element("div")?.into();
    contents.set_class_name("home button");
    contents.set_text_content(Some("Menu")); //It's not the ultimate home on deploys which has webgl1/2 menu on the root
    anchor.append_child(&contents)?;

    let anchor = anchor.unchecked_into::<HtmlAnchorElement>();
    anchor.set_href(get_home_href());

    let anchor = anchor.unchecked_into::<HtmlElement>();

    Ok(anchor)
}
fn create_source_link(href: &str, document: &Document) -> Result<HtmlElement, JsValue> {
    let anchor: HtmlElement = document.create_element("a")?.dyn_into()?;

    let contents: Element = document.create_element("div")?.into();
    contents.set_class_name("source button");
    contents.set_text_content(Some("View Source"));
    anchor.append_child(&contents)?;

    let anchor = anchor.unchecked_into::<HtmlAnchorElement>();
    anchor.set_href(&href);

    let anchor = anchor.unchecked_into::<HtmlElement>();

    Ok(anchor)
}

//Production deploys into its own /web/ folder 
//at the root
cfg_if! {
    if #[cfg(debug_assertions)] {
        pub fn get_home_href() -> &'static str {
            "/"
        }
    } else { 
        pub fn get_home_href() -> &'static str {
            "/web-demo/"
        }
    } 
}

cfg_if! {
    if #[cfg(debug_assertions)] {
        pub fn get_static_href(path: &str) -> String {
            format!("/media/{}", path)
        }
    } else { 
        pub fn get_static_href(path: &str) -> String {
            format!("/web-demo/media/{}", path)
        }
    } 
}

//Just basic stripping of urls to account for the difference in dev vs production
//giving it unicode might go wonky
fn get_root(input: &str) -> &str {
    let strip_matched = |prefix: &str| -> Option<&str> {
        input
            .find(prefix)
            .map(|len| input.split_at(len + prefix.len() - 1).1)
    };

    let stripped = strip_matched("web-demo/")
        .or(Some(input))
        .unwrap();

    stripped.trim_matches('/')
}