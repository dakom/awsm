use awsm_web::canvas::get_2d_context;
use awsm_web::errors::{Error, NativeError};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement};

pub fn generate_canvas_image(
    document: &Document,
    width: u32,
    height: u32,
    color: &str,
) -> Result<HtmlCanvasElement, Error> {
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")?
        .dyn_into()
        .map_err(|_| Error::from(NativeError::Internal))?;

    canvas.set_width(width);
    canvas.set_height(height);
    let ctx = get_2d_context(&canvas, None)?;

    ctx.set_fill_style(&JsValue::from_str(&color));
    ctx.fill_rect(0.0, 0.0, width as f64, height as f64);

    Ok(canvas)
}
