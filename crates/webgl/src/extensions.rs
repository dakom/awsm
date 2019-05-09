extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use super::renderer::*; 
use super::errors::*;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = ANGLEInstancedArrays)]
    pub type AngleInstancedArrays;

    #[wasm_bindgen(method, getter, js_name = VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE)]
    pub fn vertex_attrib_array_divisor_angle(this: &AngleInstancedArrays) -> i32;

    #[wasm_bindgen(method, catch, js_name = drawArraysInstancedANGLE)]
    pub fn draw_arrays_instanced_angle(this: &AngleInstancedArrays, mode: u32, first: i32, count: i32, primcount: i32) -> Result<(), JsValue>;

    // TODO offset should be i64
    #[wasm_bindgen(method, catch, js_name = drawElementsInstancedANGLE)]
    pub fn draw_elements_instanced_angle(this: &AngleInstancedArrays, mode: u32, count: i32, type_: u32, offset: i32, primcount: i32) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = vertexAttribDivisorANGLE)]
    pub fn vertex_attrib_divisor_angle(this: &AngleInstancedArrays, index: &web_sys::WebGlUniformLocation, divisor: u32);
}



pub fn get_extension(gl:&WebGlRenderingContext, name:&str) -> Result<js_sys::Object, Error> {
    let obj = gl.get_extension(name)?;
    obj.ok_or(Error::from(NativeError::NoExtension))
}
