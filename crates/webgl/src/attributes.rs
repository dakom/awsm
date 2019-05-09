extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlProgram, WebGlRenderingContext};
use super::enums::{DataType};
use super::errors::*;

pub struct AttributeOptions {
    pub size: i32, 
    pub data_type: DataType,
    pub normalized: bool, 
    pub stride: i32,
    // the WebIDL spec says this is actually a GLintptr or a long long
    // Rust provides functions for either u32 or f64 - and most likely
    // the f64 flavor is to allow the full Number range of JS, i.e. 52 bits
    // However - allowing float values here is probably a more likely source
    // of bugs than allowing > 52 bit values, especially since we're not concerned
    // with safety due to the wasm sandbox
    // So we're allowing the u64 type for larger values and catching accidental floats
    // It's cast to f64 to uploading (which I guess will chop the last 12 bits)
    pub offset: u64 
}

impl AttributeOptions {
    pub fn new(size: i32, data_type: DataType) -> AttributeOptions {
        AttributeOptions {
            size,
            data_type,
            normalized: false,
            stride: 0,
            offset: 0
        }
    }
}

pub fn get_attribute_location(gl:&WebGlRenderingContext, program:&WebGlProgram, name:&str) -> Result<u32, Error> {
    Some(gl.get_attrib_location(&program, &name))
        .filter(|x| *x != -1)
        .map(|x| x as u32)
        .ok_or(Error::from(NativeError::AttributeLocation))
}

pub fn activate_attribute(gl:&WebGlRenderingContext, loc:u32, opts:&AttributeOptions) {
    gl.vertex_attrib_pointer_with_f64(loc, opts.size, opts.data_type as u32, opts.normalized, opts.stride, opts.offset as f64);
    gl.enable_vertex_attrib_array(loc);
}

