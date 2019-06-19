use web_sys::{WebGlProgram, WebGlUniformLocation};
use crate::errors::{Error, NativeError};
use super::{DataType, WebGlRenderer, WebGlContext};
use log::{info};

pub enum UniformLocation<'a> {
    Name(&'a str),
    Value(WebGlUniformLocation),
}

pub fn get_uniform_location_direct(gl:&WebGlContext, program:&WebGlProgram, name:&str) -> Result<WebGlUniformLocation, Error> {
    gl.get_uniform_location(&program, &name)
        .ok_or(Error::from(NativeError::UniformLocation(Some(name.to_owned()))))
}
//Besides giving help to the typechecker, this will also automatically cast where needed
pub enum Uniform<'a, N> {
        Value1(N),
        Slice1(&'a [N]),

        Value2(N, N),
        Slice2(&'a [N]),

        Value3(N, N, N),
        Slice3(&'a [N]),
        
        Value4(N, N, N, N),
        Slice4(&'a [N]),

        Matrix2(&'a [N]),
        Matrix3(&'a [N]),
        Matrix4(&'a [N]),
        
        TransposedMatrix2(&'a [N]),
        TransposedMatrix3(&'a [N]),
        TransposedMatrix4(&'a [N]),
}

pub trait UniformData {
    fn upload(&self, gl:&WebGlContext, loc:&WebGlUniformLocation);
}

impl <'a> UniformData for Uniform<'a, f32> {
    fn upload(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        let loc = Some(loc);
        match *self {
            Uniform::Value1(a) => gl.uniform1f(loc, a),
            Uniform::Slice1(ref v) => gl.uniform1fv_with_f32_array(loc, v),
            Uniform::Value2(a, b) => gl.uniform2f(loc, a, b),
            Uniform::Slice2(ref v) => gl.uniform2fv_with_f32_array(loc, v),
            Uniform::Value3(a, b, c) => gl.uniform3f(loc, a, b, c),
            Uniform::Slice3(ref v) => gl.uniform3fv_with_f32_array(loc, v),
            Uniform::Value4(a, b, c, d) => gl.uniform4f(loc, a, b, c, d),
            Uniform::Slice4(ref v) => gl.uniform4fv_with_f32_array(loc, v),
            Uniform::Matrix2(ref v) => gl.uniform_matrix2fv_with_f32_array(loc, false, v),
            Uniform::TransposedMatrix2(ref v) => gl.uniform_matrix2fv_with_f32_array(loc, true, v),
            Uniform::Matrix3(ref v) => gl.uniform_matrix3fv_with_f32_array(loc, false, v),
            Uniform::TransposedMatrix3(ref v) => gl.uniform_matrix3fv_with_f32_array(loc, true, v),
            Uniform::Matrix4(ref v) => gl.uniform_matrix4fv_with_f32_array(loc, false, v),
            Uniform::TransposedMatrix4(ref v) => gl.uniform_matrix4fv_with_f32_array(loc, true, v),
        }
    }
}

impl <'a> UniformData for Uniform<'a, i32> {
    fn upload(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        let loc = Some(loc);
        //required due to https://github.com/rustwasm/wasm-bindgen/pull/1539
        //TODO - followup and remove when no longer needed!
        let mut values:[i32;4] = [0;4];

        //this is needed anyway for casting
        let mut matrix:[f32;16] = [0.0;16];

        //First - regular uniforms
        match *self {
            Uniform::Value1(a) => gl.uniform1i(loc, a),
            Uniform::Slice1(ref v) => {
                values[0] = v[0];
                gl.uniform1iv_with_i32_array(loc, &mut values);
            },
            Uniform::Value2(a, b) => gl.uniform2i(loc, a, b),
            Uniform::Slice2(ref v) => {
                values[0] = v[0];
                values[1] = v[1];
                gl.uniform2iv_with_i32_array(loc, &mut values);
            },

            Uniform::Value3(a, b, c) => gl.uniform3i(loc, a, b, c),
            Uniform::Slice3(ref v) => {
                values[0] = v[0];
                values[1] = v[1];
                values[2] = v[2];
                gl.uniform3iv_with_i32_array(loc, &mut values);
            },


            Uniform::Value4(a, b, c, d) => gl.uniform4i(loc, a, b, c, d),
            Uniform::Slice4(ref v) => {
                values[0] = v[0];
                values[1] = v[1];
                values[2] = v[2];
                values[3] = v[3];
                gl.uniform4iv_with_i32_array(loc, &mut values);
            },

            Uniform::Matrix2(ref v) | Uniform::TransposedMatrix2(ref v) => {
                matrix[0] = v[0] as f32;
                matrix[1] = v[1] as f32;
                matrix[2] = v[2] as f32;
                matrix[3] = v[3] as f32;
                match *self {
                    Uniform::Matrix2(_) => gl.uniform_matrix2fv_with_f32_array(loc, false, &matrix),
                    Uniform::TransposedMatrix2(_) => gl.uniform_matrix2fv_with_f32_array(loc, true, &matrix),
                    _ => {}
                }
            },
            Uniform::Matrix3(ref v) | Uniform::TransposedMatrix3(ref v) => {
                matrix[0] = v[0] as f32;
                matrix[1] = v[1] as f32;
                matrix[2] = v[2] as f32;
                matrix[3] = v[3] as f32;
                matrix[4] = v[4] as f32;
                matrix[5] = v[5] as f32;
                matrix[6] = v[6] as f32;
                matrix[7] = v[7] as f32;
                matrix[8] = v[8] as f32;
                match *self {
                    Uniform::Matrix3(_) => gl.uniform_matrix3fv_with_f32_array(loc, false, &matrix),
                    Uniform::TransposedMatrix3(_) => gl.uniform_matrix3fv_with_f32_array(loc, true, &matrix),
                    _ => {}
                }
            },
            Uniform::Matrix4(ref v) | Uniform::TransposedMatrix4(ref v) => {
                matrix[0] = v[0] as f32;
                matrix[1] = v[1] as f32;
                matrix[2] = v[2] as f32;
                matrix[3] = v[3] as f32;
                matrix[4] = v[4] as f32;
                matrix[5] = v[5] as f32;
                matrix[6] = v[6] as f32;
                matrix[7] = v[7] as f32;
                matrix[8] = v[8] as f32;
                matrix[9] = v[9] as f32;
                matrix[10] = v[10] as f32;
                matrix[11] = v[11] as f32;
                matrix[12] = v[12] as f32;
                matrix[13] = v[13] as f32;
                matrix[14] = v[14] as f32;
                matrix[15] = v[15] as f32;
                match *self {
                    Uniform::Matrix4(_) => gl.uniform_matrix4fv_with_f32_array(loc, false, &matrix),
                    Uniform::TransposedMatrix4(_) => gl.uniform_matrix4fv_with_f32_array(loc, true, &matrix),
                    _ => {}
                }
            }
        }
    }
}

impl <'a> UniformData for Uniform<'a, i64> {
    fn upload(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        let loc = Some(loc);
        //required due to casting 
        let mut values:[i32;4] = [0;4];
        let mut matrix:[f32;16] = [0.0;16];

        match *self {
            Uniform::Value1(a) => gl.uniform1i(loc, a as i32),
            Uniform::Slice1(ref v) => {
                values[0] = v[0] as i32;
                gl.uniform1iv_with_i32_array(loc, &mut values);
            },
            Uniform::Value2(a, b) => gl.uniform2i(loc, a as i32, b as i32),
            Uniform::Slice2(ref v) => {
                values[0] = v[0] as i32;
                values[1] = v[1] as i32;
                gl.uniform2iv_with_i32_array(loc, &mut values);
            },

            Uniform::Value3(a, b, c) => gl.uniform3i(loc, a as i32, b as i32, c as i32),
            Uniform::Slice3(ref v) => {
                values[0] = v[0] as i32;
                values[1] = v[1] as i32;
                values[2] = v[2] as i32;
                gl.uniform3iv_with_i32_array(loc, &mut values);
            },

            Uniform::Value4(a, b, c, d) => gl.uniform4i(loc, a as i32, b as i32, c as i32, d as i32),
            Uniform::Slice4(ref v) => {
                values[0] = v[0] as i32;
                values[1] = v[1] as i32;
                values[2] = v[2] as i32;
                values[3] = v[3] as i32;
                gl.uniform4iv_with_i32_array(loc, &mut values);
            }


            Uniform::Matrix2(ref v) | Uniform::TransposedMatrix2(ref v) => {
                matrix[0] = v[0] as f32;
                matrix[1] = v[1] as f32;
                matrix[2] = v[2] as f32;
                matrix[3] = v[3] as f32;
                match *self {
                    Uniform::Matrix2(_) => gl.uniform_matrix2fv_with_f32_array(loc, false, &matrix),
                    Uniform::TransposedMatrix2(_) => gl.uniform_matrix2fv_with_f32_array(loc, true, &matrix),
                    _ => {}
                }
            },
            Uniform::Matrix3(ref v) | Uniform::TransposedMatrix3(ref v) => {
                matrix[0] = v[0] as f32;
                matrix[1] = v[1] as f32;
                matrix[2] = v[2] as f32;
                matrix[3] = v[3] as f32;
                matrix[4] = v[4] as f32;
                matrix[5] = v[5] as f32;
                matrix[6] = v[6] as f32;
                matrix[7] = v[7] as f32;
                matrix[8] = v[8] as f32;
                match *self {
                    Uniform::Matrix3(_) => gl.uniform_matrix3fv_with_f32_array(loc, false, &matrix),
                    Uniform::TransposedMatrix3(_) => gl.uniform_matrix3fv_with_f32_array(loc, true, &matrix),
                    _ => {}
                }
            },
            Uniform::Matrix4(ref v) | Uniform::TransposedMatrix4(ref v) => {
                matrix[0] = v[0] as f32;
                matrix[1] = v[1] as f32;
                matrix[2] = v[2] as f32;
                matrix[3] = v[3] as f32;
                matrix[4] = v[4] as f32;
                matrix[5] = v[5] as f32;
                matrix[6] = v[6] as f32;
                matrix[7] = v[7] as f32;
                matrix[8] = v[8] as f32;
                matrix[9] = v[9] as f32;
                matrix[10] = v[10] as f32;
                matrix[11] = v[11] as f32;
                matrix[12] = v[12] as f32;
                matrix[13] = v[13] as f32;
                matrix[14] = v[14] as f32;
                matrix[15] = v[15] as f32;
                match *self {
                    Uniform::Matrix4(_) => gl.uniform_matrix4fv_with_f32_array(loc, false, &matrix),
                    Uniform::TransposedMatrix4(_) => gl.uniform_matrix4fv_with_f32_array(loc, true, &matrix),
                    _ => {}
                }
            }
        }
    }
}


impl <'a> UniformData for Uniform<'a, f64> {
    fn upload(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        let loc = Some(loc);
        //required due to casting 
        let mut values:[f32;4] = [0.0;4];
        let mut matrix:[f32;16] = [0.0;16];

        match *self {
            Uniform::Value1(a) => gl.uniform1f(loc, a as f32),
            Uniform::Slice1(ref v) => {
                values[0] = v[0] as f32;
                gl.uniform1fv_with_f32_array(loc, &mut values);
            },
            Uniform::Value2(a, b) => gl.uniform2f(loc, a as f32, b as f32),
            Uniform::Slice2(ref v) => {
                values[0] = v[0] as f32;
                values[1] = v[1] as f32;
                gl.uniform2fv_with_f32_array(loc, &mut values);
            },

            Uniform::Value3(a, b, c) => gl.uniform3f(loc, a as f32, b as f32, c as f32),
            Uniform::Slice3(ref v) => {
                values[0] = v[0] as f32;
                values[1] = v[1] as f32;
                values[2] = v[2] as f32;
                gl.uniform3fv_with_f32_array(loc, &mut values);
            },


            Uniform::Value4(a, b, c, d) => gl.uniform4f(loc, a as f32, b as f32, c as f32, d as f32),
            Uniform::Slice4(ref v) => {
                values[0] = v[0] as f32;
                values[1] = v[1] as f32;
                values[2] = v[2] as f32;
                values[3] = v[3] as f32;
                gl.uniform4fv_with_f32_array(loc, &mut values);
            }


            Uniform::Matrix2(ref v) | Uniform::TransposedMatrix2(ref v) => {
                matrix[0] = v[0] as f32;
                matrix[1] = v[1] as f32;
                matrix[2] = v[2] as f32;
                matrix[3] = v[3] as f32;
                match *self {
                    Uniform::Matrix2(_) => gl.uniform_matrix2fv_with_f32_array(loc, false, &matrix),
                    Uniform::TransposedMatrix2(_) => gl.uniform_matrix2fv_with_f32_array(loc, true, &matrix),
                    _ => {}
                }
            },
            Uniform::Matrix3(ref v) | Uniform::TransposedMatrix3(ref v) => {
                matrix[0] = v[0] as f32;
                matrix[1] = v[1] as f32;
                matrix[2] = v[2] as f32;
                matrix[3] = v[3] as f32;
                matrix[4] = v[4] as f32;
                matrix[5] = v[5] as f32;
                matrix[6] = v[6] as f32;
                matrix[7] = v[7] as f32;
                matrix[8] = v[8] as f32;
                match *self {
                    Uniform::Matrix3(_) => gl.uniform_matrix3fv_with_f32_array(loc, false, &matrix),
                    Uniform::TransposedMatrix3(_) => gl.uniform_matrix3fv_with_f32_array(loc, true, &matrix),
                    _ => {}
                }
            },
            Uniform::Matrix4(ref v) | Uniform::TransposedMatrix4(ref v) => {
                matrix[0] = v[0] as f32;
                matrix[1] = v[1] as f32;
                matrix[2] = v[2] as f32;
                matrix[3] = v[3] as f32;
                matrix[4] = v[4] as f32;
                matrix[5] = v[5] as f32;
                matrix[6] = v[6] as f32;
                matrix[7] = v[7] as f32;
                matrix[8] = v[8] as f32;
                matrix[9] = v[9] as f32;
                matrix[10] = v[10] as f32;
                matrix[11] = v[11] as f32;
                matrix[12] = v[12] as f32;
                matrix[13] = v[13] as f32;
                matrix[14] = v[14] as f32;
                matrix[15] = v[15] as f32;
                match *self {
                    Uniform::Matrix4(_) => gl.uniform_matrix4fv_with_f32_array(loc, false, &matrix),
                    Uniform::TransposedMatrix4(_) => gl.uniform_matrix4fv_with_f32_array(loc, true, &matrix),
                    _ => {}
                }
            }
        }
    }
}

impl WebGlRenderer {
    pub fn get_uniform_location_value(&self, name:&str) -> Result<WebGlUniformLocation, Error> {

        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        program_info.uniform_lookup
            .get(name)
            .map(|v| v.clone())
            .ok_or_else(|| Error::from(NativeError::UniformLocation(Some(name.to_string()))))
    }


    pub fn upload_uniform<T>(&mut self, loc:&UniformLocation, data:&T) -> Result<(), Error> 
    where T: UniformData
    {
        let loc = match loc {
            UniformLocation::Name(ref name) => {
                self.get_uniform_location_value(&name)?
            },
            UniformLocation::Value(ref loc) => {
                loc.clone()
            }
        };
        
        data.upload(&self.gl, &loc);
        Ok(())
    }
}
