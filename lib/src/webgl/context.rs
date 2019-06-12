use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext, WebGl2RenderingContext};
use crate::errors::{Error, NativeError};
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "webgl1")] {
        pub type WebGlContext = WebGlRenderingContext;
        pub fn get_context(canvas:&HtmlCanvasElement) -> Result<WebGlContext, Error> {
            canvas.get_context("webgl1")
                .and_then(|obj| 
                          match obj {
                              None => Err(Error::Empty.into()),
                              Some(ctx) => 
                                  ctx.dyn_into::<web_sys::WebGlRenderingContext>()
                                  .map_err(|err| err.into())
                          }
                )
                .map_err(|_| Error::Native(NativeError::WebGlContext))
        }
    } else {
        pub type WebGlContext = WebGl2RenderingContext;
        pub fn get_context(canvas:&HtmlCanvasElement) -> Result<WebGlContext, Error> {
            canvas.get_context("webgl2")
                .and_then(|obj| 
                          match obj {
                              None => Err(Error::Empty.into()),
                              Some(ctx) => 
                                  ctx.dyn_into::<web_sys::WebGl2RenderingContext>()
                                  .map_err(|err| err.into())
                          }
                )
                .map_err(|_| Error::Native(NativeError::WebGlContext))
        }
    }
}


