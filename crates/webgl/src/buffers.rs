extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlBuffer, WebGlRenderingContext};
use js_sys::{WebAssembly};
use wasm_bindgen::JsCast;
use super::errors::*;
use super::enums::{BufferTarget, BufferUsage};

pub fn upload_array_buffer(gl:&WebGlRenderingContext, values:&[f32], target: BufferTarget, usage:BufferUsage, webgl_buffer:&WebGlBuffer) -> Result<(), Error> {
    wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .map(|m:WebAssembly::Memory| {

            let wasm_buffer = m.buffer();
            let ptr_loc = values.as_ptr() as u32 / 4;

            let float32 = js_sys::Float32Array::new(&wasm_buffer)
                            .subarray(ptr_loc, ptr_loc + values.len() as u32);
    
            //Note - WebGL2 can do less GC hits by pointing at same memory with different start/end
            gl.buffer_data_with_array_buffer_view(target as u32, &float32, usage as u32); 
            
        })
        .map_err(|err| Error::from(err))
}

pub fn bind_buffer(gl:&WebGlRenderingContext, target:BufferTarget, buffer:&WebGlBuffer) {
    gl.bind_buffer(target as u32, Some(buffer)); 
}
