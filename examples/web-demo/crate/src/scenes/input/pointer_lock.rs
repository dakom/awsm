use awsm_web::input::PointerLock;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window};

pub fn start(_window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {
    let bg: Element = document.create_element("div")?.into();
    bg.set_class_name("pointer-lock-container");
    body.append_child(&bg)?;

    let button: HtmlElement = document.create_element("div")?.dyn_into()?;
    button.set_class_name("button pointer-lock-button");
    button.set_text_content(Some("click to start"));
    body.append_child(&button)?;

    let cursor: HtmlElement = document.create_element("div")?.dyn_into()?;
    cursor.set_class_name("pointer-lock-cursor");
    body.append_child(&cursor)?;

    let cursor_style = Rc::new(cursor.style());
    let toggle_cursor = {
        let button_style = button.style();
        let cursor_style = cursor_style.clone();
        Rc::new(move |flag: bool| match flag {
            true => {
                cursor_style.set_property("display", "block").unwrap();
                button_style.set_property("display", "none").unwrap();
            }
            false => {
                cursor_style.set_property("display", "none").unwrap();
                button_style.set_property("display", "block").unwrap();
            }
        })
    };

    toggle_cursor(false);

    let pos_x = Rc::new(Cell::new(0));
    let pos_y = Rc::new(Cell::new(0));
    let draw = {
        let pos_x = pos_x.clone();
        let pos_y = pos_y.clone();
        let cursor_style = cursor_style.clone();
        Rc::new(move || {
            cursor_style
                .set_property("left", format!("{}px", pos_x.get()).as_ref())
                .unwrap();
            cursor_style
                .set_property("top", format!("{}px", pos_y.get()).as_ref())
                .unwrap();
        })
    };

    let on_start = {
        let pos_x = pos_x.clone();
        let pos_y = pos_y.clone();
        let draw = draw.clone();
        let toggle_cursor = toggle_cursor.clone();
        move |e: &Element| {
            toggle_cursor(true);
            let bg: HtmlElement = e.clone().dyn_into().unwrap();
            let (x, y) = (bg.client_width() / 2, bg.client_height() / 2);
            pos_x.set(x);
            pos_y.set(y);
            draw();
        }
    };
    let on_move = {
        let pos_x = pos_x.clone();
        let pos_y = pos_y.clone();
        let draw = draw.clone();
        move |dx: i32, dy: i32| {
            let x = pos_x.get() + dx;
            let y = pos_y.get() + dy;
            pos_x.set(x);
            pos_y.set(y);
            draw();
        }
    };

    let on_end = {
        let toggle_cursor = toggle_cursor.clone();
        move || {
            toggle_cursor(false);
        }
    };

    let pointer_lock = PointerLock::start(&button, &bg, &document, on_start, on_move, on_end);

    std::mem::forget(Box::new(pointer_lock));
    Ok(())
}
