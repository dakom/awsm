extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::JsCast;

pub fn get_canvas_context_1(canvas:&web_sys::HtmlCanvasElement) -> Option<web_sys::WebGlRenderingContext> {
    canvas
        .get_context("webgl")
        .ok()
        .and_then(|object| object)
        .and_then(|object| 
                object.dyn_into::<web_sys::WebGlRenderingContext>().ok()
        )
}

pub fn get_canvas_context_2(canvas:&web_sys::HtmlCanvasElement) -> Option<web_sys::WebGl2RenderingContext> {
    canvas
        .get_context("webgl2")
        .ok()
        .and_then(|object| object)
        .and_then(|object| 
                object.dyn_into::<web_sys::WebGl2RenderingContext>().ok()
        )
}
