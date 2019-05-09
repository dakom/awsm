extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};
use super::enums::{DataType};
use super::errors::*;
use super::renderer::*; 
use super::errors::*;

pub enum UniformData<'a> {
        FloatVal1(f32),
        Float1(&'a [f32]),
        IntVal1(i32),
        Int1(&'a [i32]),

        FloatVal2(f32, f32),
        Float2(&'a [f32]),
        IntVal2(i32, i32),
        Int2(&'a [i32]),

        FloatVal3(f32, f32, f32),
        Float3(&'a [f32]),
        IntVal3(i32, i32, i32),
        Int3(&'a [i32]),

        FloatVal4(f32, f32, f32, f32),
        Float4(&'a [f32]),
        IntVal4(i32, i32, i32, i32),
        Int4(&'a [i32]),
}

pub enum UniformMatrixData <'a> {
        Float2(&'a [f32]),
        Float3(&'a [f32]),
        Float4(&'a [f32]),

        Float2Transposed(&'a [f32]),
        Float3Transposed(&'a [f32]),
        Float4Transposed(&'a [f32]),
}

pub fn get_uniform_location(gl:&WebGlRenderingContext, program:&WebGlProgram, name:&str) -> Result<WebGlUniformLocation, Error> {
    gl.get_uniform_location(&program, &name)
        .ok_or(Error::from(NativeError::UniformLocation))
}

pub fn set_uniform_data(gl:&WebGlRenderingContext, loc:&WebGlUniformLocation, data: UniformData) {
        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.uniform1f

        //the only because the gl.uniform calls require passing a mutable matrix
        //see https://github.com/rustwasm/wasm-bindgen/issues/1131
        //and https://github.com/rustwasm/wasm-bindgen/issues/1005
        let mut mutable_v:[i32;4] = [0, 0, 0, 0];

        let loc = Some(loc);
        match data {
                UniformData::FloatVal1(a) => gl.uniform1f(loc, a),
                UniformData::Float1(v) => gl.uniform1fv_with_f32_array(loc, v),
                UniformData::IntVal1(a) => gl.uniform1i(loc, a),
                UniformData::Int1(v) => {
                        mutable_v[0] = v[0];
                        gl.uniform1iv_with_i32_array(loc, &mut mutable_v);
                }
                
                UniformData::FloatVal2(a, b) => gl.uniform2f(loc, a, b),
                UniformData::Float2(v) => gl.uniform2fv_with_f32_array(loc, v),
                UniformData::IntVal2(a, b) => gl.uniform2i(loc, a, b),
                UniformData::Int2(v) => {
                        mutable_v[0] = v[0];
                        mutable_v[1] = v[1];
                        gl.uniform2iv_with_i32_array(loc, &mut mutable_v);
                }

                UniformData::FloatVal3(a, b, c) => gl.uniform3f(loc, a, b, c),
                UniformData::Float3(v) => gl.uniform3fv_with_f32_array(loc, v),
                UniformData::IntVal3(a, b, c) => gl.uniform3i(loc, a, b, c),
                UniformData::Int3(v) => {
                        mutable_v[0] = v[0];
                        mutable_v[1] = v[1];
                        mutable_v[2] = v[2];
                        gl.uniform3iv_with_i32_array(loc, &mut mutable_v);
                }

                UniformData::FloatVal4(a, b, c, d) => gl.uniform4f(loc, a, b, c, d),
                UniformData::Float4(v) => gl.uniform4fv_with_f32_array(loc, v),
                UniformData::IntVal4(a, b, c, d) => gl.uniform4i(loc, a, b, c, d),
                UniformData::Int4(v) => {
                        mutable_v[0] = v[0];
                        mutable_v[1] = v[1];
                        mutable_v[2] = v[2];
                        mutable_v[3] = v[3];
                        gl.uniform4iv_with_i32_array(loc, &mut mutable_v);
                }

                _ => {} 
        }
}
pub fn set_uniform_matrix_data(gl:&WebGlRenderingContext, loc:&WebGlUniformLocation, data: UniformMatrixData) {
        let loc = Some(loc);

        match &data {
                UniformMatrixData::Float2(v) => gl.uniform_matrix2fv_with_f32_array(loc, false, v),
                UniformMatrixData::Float3(v) => gl.uniform_matrix3fv_with_f32_array(loc, false, v),
                UniformMatrixData::Float4(v) => gl.uniform_matrix4fv_with_f32_array(loc, false, v),

                UniformMatrixData::Float2Transposed(v) => gl.uniform_matrix2fv_with_f32_array(loc, true, v),
                UniformMatrixData::Float3Transposed(v) => gl.uniform_matrix3fv_with_f32_array(loc, true, v),
                UniformMatrixData::Float4Transposed(v) => gl.uniform_matrix4fv_with_f32_array(loc, true, v),
                _ => {} 
        }
}