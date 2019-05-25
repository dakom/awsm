use wasm_bindgen::prelude::*;
use super::menu;
use super::scenes::*;
use wasm_bindgen::JsCast;

// Called by our JS entry point to run the example.
pub fn start_router(window:web_sys::Window, document:web_sys::Document) -> Result<(), JsValue> {
    let body = document.body().expect("should have body");

    let pathname = window.location().pathname()?;

    let pathname = get_root(pathname.as_str());

    match pathname {
        "" => {
            let menu = menu::build_menu(&document)?;
            body.append_child(&menu)?;
            Ok(())
        },

        "clock" => {
            clock::start(window, document, body)
        }

        "webgl-simple" => {
            quad::simple::start(window, document, body)
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
