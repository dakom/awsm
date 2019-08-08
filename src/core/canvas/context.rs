use crate::errors::{Error, NativeError};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Canvas2dContextOptions {
    alpha: bool,
    desynchronized: bool,
}

impl Default for Canvas2dContextOptions {
    fn default() -> Self {
        Self {
            alpha: true,
            desynchronized: false,
        }
    }
}

impl Canvas2dContextOptions {
    pub fn to_js_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

pub type Canvas2dContext = CanvasRenderingContext2d;

pub fn get_2d_context(
    canvas: &HtmlCanvasElement,
    opts: Option<&Canvas2dContextOptions>,
) -> Result<Canvas2dContext, Error> {
    let context = match opts {
        Some(opts) => canvas.get_context_with_context_options("2d", &opts.to_js_value()),
        None => canvas.get_context("2d"),
    };
    context
        .and_then(|obj| match obj {
            None => Err(Error::Empty.into()),
            Some(ctx) => ctx
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .map_err(|err| err.into()),
        })
        .map_err(|_| Error::Native(NativeError::Canvas2dContext))
}
