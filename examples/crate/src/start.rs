use wasm_bindgen::prelude::*;
use super::menu;
use wasm_bindgen::JsCast;

// Called by our JS entry point to run the example.
pub fn start(window:web_sys::Window, document:web_sys::Document) -> Result<(), JsValue> {
    let body = document.body().expect("should have body");

    let pathname = window.location().pathname()?;

    let pathname = pathname.as_str();

    match pathname {
        "/" => {
            let menu = menu::build_menu(&document)?;
            body.append_child(&menu)?;
            Ok(())
        },

        "/clock" => {
            super::scenes::clock::clock::start(window, document, body)
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

