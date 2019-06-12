use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::errors::{Error, NativeError};
use super::context::{WebGlContext};

//Not actually used atm because we're defaulting to WebGL2
//But kept here for a working example of how extensions could be implemented
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
    pub fn vertex_attrib_divisor_angle(this: &AngleInstancedArrays, loc: u32, divisor: u32);
}



pub fn get_extension(gl:&WebGlContext, name:&str) -> Result<js_sys::Object, Error> {
    let obj = gl.get_extension(name)?;
    obj.ok_or(Error::from(NativeError::NoExtension))
}
