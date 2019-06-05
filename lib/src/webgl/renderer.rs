use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use log::{info};

pub struct WebGlRenderer {
    pub gl:WebGl2RenderingContext,
    pub canvas: HtmlCanvasElement,
    last_width: u32,
    last_height: u32
}

impl WebGlRenderer {
    pub fn new(canvas:HtmlCanvasElement) -> Result<Self, &'static str> {

        let gl = canvas.get_context("webgl2")
                .and_then(|obj| 
                    match obj {
                        None => Err(JsValue::null()),
                        Some(ctx) => 
                            ctx.dyn_into::<web_sys::WebGl2RenderingContext>()
                                .map_err(|err| err.into())
                    }
                )
                .map_err(|_| "couldn't get context")?;

        Ok(
            Self {
                gl,
                canvas,
                last_width: 0,
                last_height: 0
            }
        )
    }

    pub fn resize(&mut self, width:u32, height:u32) {
        if self.last_width != width || self.last_height != height {
            info!("resizing: {},{}", width, height);

            let gl = &mut self.gl;
            let canvas = &mut self.canvas;
            canvas.set_width(width);
            canvas.set_height(height);
            gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());
            self.last_width = width;
            self.last_height = height;
        }
    }
}
