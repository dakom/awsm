use awsm::webgl::{ClearBufferMask, WebGlRenderer, UniformMatrixData, UniformData, BeginMode};
use awsm::helpers::*;
use awsm::camera::{write_ortho};
use awsm::tick::{start_raf_ticker_timestamp, Timestamp};
use std::rc::Rc; 
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{Window, Document, HtmlElement};
use crate::scenes::webgl::common::{start_webgl, create_unit_box_buffers}; 

//TODO - match https://github.com/dakom/pure3d-typescript/blob/master/examples/src/app/scenes/basic/box/box-vao-renderer/Box-Vao-Renderer.ts
//
pub fn start(window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {
    Ok(())
}
