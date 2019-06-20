use web_sys::{WebGlProgram, WebGlUniformLocation};
use crate::errors::{Error, NativeError};
use super::{DataType, WebGlRenderer, WebGlContext};
use log::{info};
use std::mem;

//TODO 
//1. followup with https://github.com/rustwasm/wasm-bindgen/pull/1539
//When the i32 slices don't need mut anymore - simplify below

//2. Rewrite all the below with macros!

pub enum Uniform<'a> {
    Name(&'a str),
    Loc(WebGlUniformLocation),
}

pub struct UniformData<T>(pub T);


pub fn get_uniform_location_direct(gl:&WebGlContext, program:&WebGlProgram, name:&str) -> Result<WebGlUniformLocation, Error> {
    gl.get_uniform_location(&program, &name)
        .ok_or(Error::from(NativeError::UniformLocation(Some(name.to_owned()))))
}


/*
 * The direct uniform uploads are written as traits in order to allow working either f32 or i32
 * 
 * Arrays, scalars, and tuples are guaranteed at compile-time to be the right size
 * This also allows them to use the generic upload_uniform_[values/slice/matrix/matrix_transposed]
 * Which doesn't even return a Result (since none is returned from the JS API)
 *
 * Slices require the user to specify the size (e.g. upload_unifrm_*_N()).
 * These variants do return a Result (and check that the slice has enough elements)
 */

pub trait UniformValues {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation);
}

pub trait UniformValues_1 {
    fn upload_uniform_values_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;
}
pub trait UniformValues_2 {
    fn upload_uniform_values_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;
}
pub trait UniformValues_3 {
    fn upload_uniform_values_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;
}
pub trait UniformValues_4 {
    fn upload_uniform_values_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;

}
pub trait UniformSlice {
    fn upload_uniform_slice(&self, gl:&WebGlContext, loc:&WebGlUniformLocation);
}

pub trait UniformSlice_1 {
    fn upload_uniform_slice_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;

}
pub trait UniformSlice_2 {
    fn upload_uniform_slice_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;

}
pub trait UniformSlice_3 {
    fn upload_uniform_slice_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;

}
pub trait UniformSlice_4 {
    fn upload_uniform_slice_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;

}
pub trait UniformMatrix {
    fn upload_uniform_matrix(&self, gl:&WebGlContext, loc:&WebGlUniformLocation);
    fn upload_uniform_matrix_transposed(&self, gl:&WebGlContext, loc:&WebGlUniformLocation);
}

pub trait UniformMatrix_2 {
    fn upload_uniform_matrix_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;

    fn upload_uniform_matrix_transposed_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;

}
pub trait UniformMatrix_3 {
    fn upload_uniform_matrix_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;

    fn upload_uniform_matrix_transposed_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;

}
pub trait UniformMatrix_4 {
    fn upload_uniform_matrix_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;

    fn upload_uniform_matrix_transposed_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error>;

}


impl UniformValues for UniformData<f32> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform1f(Some(loc), self.0);
    }
}
impl UniformValues for UniformData<i32> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform1i(Some(loc), self.0);
    }
}
impl UniformValues_1 for UniformData<f32> {
    fn upload_uniform_values_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform1f(Some(loc), self.0);
        Ok(())
    }
}
impl UniformValues_1 for UniformData<i32> {
    fn upload_uniform_values_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform1i(Some(loc), self.0);
        Ok(())
    }
}

impl UniformValues for UniformData<(f32, f32)> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform2f(Some(loc), (self.0).0, (self.0).1);
    }
}
impl UniformValues for UniformData<(i32, i32)> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform2i(Some(loc), (self.0).0, (self.0).1);
    }
}
impl UniformValues_2 for UniformData<(f32, f32)> {
    fn upload_uniform_values_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform2f(Some(loc), (self.0).0, (self.0).1);
        Ok(())
    }
}
impl UniformValues_2 for UniformData<(i32, i32)> {
    fn upload_uniform_values_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform2i(Some(loc), (self.0).0, (self.0).1);
        Ok(())
    }
}

impl UniformValues for UniformData<(f32, f32, f32)> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform3f(Some(loc), (self.0).0, (self.0).1, (self.0).2);
    }
}
impl UniformValues for UniformData<(i32, i32, i32)> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform3i(Some(loc), (self.0).0, (self.0).1, (self.0).2);
    }
}
impl UniformValues_3 for UniformData<(f32, f32, f32)> {
    fn upload_uniform_values_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform3f(Some(loc), (self.0).0, (self.0).1, (self.0).2);
        Ok(())
    }
}
impl UniformValues_3 for UniformData<(i32, i32, i32)> {
    fn upload_uniform_values_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform3i(Some(loc), (self.0).0, (self.0).1, (self.0).2);
        Ok(())
    }
}

impl UniformValues for UniformData<(f32, f32, f32, f32)> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform4f(Some(loc), (self.0).0, (self.0).1, (self.0).2, (self.0).3);
    }
}
impl UniformValues for UniformData<(i32, i32, i32, i32)> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform4i(Some(loc), (self.0).0, (self.0).1, (self.0).2, (self.0).3);
    }
}
impl UniformValues_4 for UniformData<(f32, f32, f32, f32)> {
    fn upload_uniform_values_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform4f(Some(loc), (self.0).0, (self.0).1, (self.0).2, (self.0).3);
        Ok(())
    }
}
impl UniformValues_4 for UniformData<(i32, i32, i32, i32)> {
    fn upload_uniform_values_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform4i(Some(loc), (self.0).0, (self.0).1, (self.0).2, (self.0).3);
        Ok(())
    }
}

impl UniformValues for UniformData<[f32;1]> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform1f(Some(loc), self.0[0]);
    }
}
impl UniformValues for UniformData<[i32;1]> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform1i(Some(loc), self.0[0]);
    }
}
impl UniformValues_1 for UniformData<[f32;1]> {
    fn upload_uniform_values_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform1f(Some(loc), self.0[0]);
        Ok(())
    }
}
impl UniformValues_1 for UniformData<[i32;1]> {
    fn upload_uniform_values_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform1i(Some(loc), self.0[0]);
        Ok(())
    }
}

impl UniformValues for UniformData<[f32;2]> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform2f(Some(loc), self.0[0], self.0[1]);
    }
}
impl UniformValues for UniformData<[i32;2]> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform2i(Some(loc), self.0[0], self.0[1]);
    }
}
impl UniformValues_2 for UniformData<[f32;2]> {
    fn upload_uniform_values_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform2f(Some(loc), self.0[0], self.0[1]);
        Ok(())
    }
}
impl UniformValues_2 for UniformData<[i32;2]> {
    fn upload_uniform_values_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform2i(Some(loc), self.0[0], self.0[1]);
        Ok(())
    }
}

impl UniformValues for UniformData<[f32;3]> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform3f(Some(loc), self.0[0], self.0[1], self.0[2]);
    }
}
impl UniformValues for UniformData<[i32;3]> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform3i(Some(loc), self.0[0], self.0[1], self.0[2]);
    }
}
impl UniformValues_3 for UniformData<[f32;3]> {
    fn upload_uniform_values_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform3f(Some(loc), self.0[0], self.0[1], self.0[2]);
        Ok(())
    }
}
impl UniformValues_3 for UniformData<[i32;3]> {
    fn upload_uniform_values_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform3i(Some(loc), self.0[0], self.0[1], self.0[2]);
        Ok(())
    }
}


impl UniformValues for UniformData<[f32;4]> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform4f(Some(loc), self.0[0], self.0[1], self.0[2], self.0[3]);
    }
}
impl UniformValues for UniformData<[i32;4]> {
    fn upload_uniform_values(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform4i(Some(loc), self.0[0], self.0[1], self.0[2], self.0[3]);
    }
}
impl UniformValues_4 for UniformData<[f32;4]> {
    fn upload_uniform_values_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform4f(Some(loc), self.0[0], self.0[1], self.0[2], self.0[3]);
        Ok(())
    }
}
impl UniformValues_4 for UniformData<[i32;4]> {
    fn upload_uniform_values_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform4i(Some(loc), self.0[0], self.0[1], self.0[2], self.0[3]);
        Ok(())
    }
}

impl UniformValues_1 for UniformData<&[f32]> {
    fn upload_uniform_values_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 1 {
            gl.uniform1f(Some(loc), self.0[0]);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformValues_1 for UniformData<&[i32]> {
    fn upload_uniform_values_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 1 {
            gl.uniform1i(Some(loc), self.0[0]);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}

impl UniformValues_2 for UniformData<&[f32]> {
    fn upload_uniform_values_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 2 {
            gl.uniform2f(Some(loc), self.0[0], self.0[1]);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformValues_2 for UniformData<&[i32]> {
    fn upload_uniform_values_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 2 {
            gl.uniform2i(Some(loc), self.0[0], self.0[1]);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformValues_3 for UniformData<&[f32]> {
    fn upload_uniform_values_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 3 {
            gl.uniform3f(Some(loc), self.0[0], self.0[1], self.0[2]);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformValues_3 for UniformData<&[i32]> {
    fn upload_uniform_values_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 3 {
            gl.uniform3i(Some(loc), self.0[0], self.0[1], self.0[2]);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}

impl UniformValues_4 for UniformData<&[f32]> {
    fn upload_uniform_values_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 4 {
            gl.uniform4f(Some(loc), self.0[0], self.0[1], self.0[2], self.0[3]);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformValues_4 for UniformData<&[i32]> {
    fn upload_uniform_values_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 4 {
            gl.uniform4i(Some(loc), self.0[0], self.0[1], self.0[2], self.0[3]);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
/*
 * Uniform Slice
 */

impl UniformSlice for UniformData<[f32;1]> {
    fn upload_uniform_slice(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform1fv_with_f32_array(Some(loc), &self.0);
    }
}
impl UniformSlice for UniformData<[i32;1]> {
    fn upload_uniform_slice(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        let mut values:[i32;1] = [self.0[0]];
        gl.uniform1iv_with_i32_array(Some(loc), &mut values);
    }
}
impl UniformSlice_1 for UniformData<[f32;1]> {
    fn upload_uniform_slice_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform1fv_with_f32_array(Some(loc), &self.0);
        Ok(())
    }
}
impl UniformSlice_1 for UniformData<[i32;1]> {
    fn upload_uniform_slice_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        let mut values:[i32;1] = [self.0[0]];
        gl.uniform1iv_with_i32_array(Some(loc), &mut values);
        Ok(())
    }
}


impl UniformSlice for UniformData<[f32;2]> {
    fn upload_uniform_slice(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform2fv_with_f32_array(Some(loc), &self.0);
    }
}
impl UniformSlice for UniformData<[i32;2]> {
    fn upload_uniform_slice(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        let mut values:[i32;2] = [self.0[0], self.0[1]];
        gl.uniform2iv_with_i32_array(Some(loc), &mut values);
    }
}
impl UniformSlice_2 for UniformData<[f32;2]> {
    fn upload_uniform_slice_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform2fv_with_f32_array(Some(loc), &self.0);
        Ok(())
    }
}
impl UniformSlice_2 for UniformData<[i32;2]> {
    fn upload_uniform_slice_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        let mut values:[i32;2] = [self.0[0], self.0[1]];
        gl.uniform2iv_with_i32_array(Some(loc), &mut values);
        Ok(())
    }
}


impl UniformSlice for UniformData<[f32;3]> {
    fn upload_uniform_slice(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform3fv_with_f32_array(Some(loc), &self.0);
    }
}
impl UniformSlice for UniformData<[i32;3]> {
    fn upload_uniform_slice(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        let mut values:[i32;3] = [self.0[0], self.0[1], self.0[2]];
        gl.uniform3iv_with_i32_array(Some(loc), &mut values);
    }
}
impl UniformSlice_3 for UniformData<[f32;3]> {
    fn upload_uniform_slice_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform3fv_with_f32_array(Some(loc), &self.0);
        Ok(())
    }
}
impl UniformSlice_3 for UniformData<[i32;3]> {
    fn upload_uniform_slice_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        let mut values:[i32;3] = [self.0[0], self.0[1], self.0[2]];
        gl.uniform3iv_with_i32_array(Some(loc), &mut values);
        Ok(())
    }
}

impl UniformSlice for UniformData<[f32;4]> {
    fn upload_uniform_slice(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform4fv_with_f32_array(Some(loc), &self.0);
    }
}
impl UniformSlice for UniformData<[i32;4]> {
    fn upload_uniform_slice(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        let mut values:[i32;4] = [self.0[0], self.0[1], self.0[2], self.0[3]];
        gl.uniform4iv_with_i32_array(Some(loc), &mut values);
    }
}
impl UniformSlice_4 for UniformData<[f32;4]> {
    fn upload_uniform_slice_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform4fv_with_f32_array(Some(loc), &self.0);
        Ok(())
    }
}
impl UniformSlice_4 for UniformData<[i32;4]> {
    fn upload_uniform_slice_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        let mut values:[i32;4] = [self.0[0], self.0[1], self.0[2], self.0[3]];
        gl.uniform4iv_with_i32_array(Some(loc), &mut values);
        Ok(())
    }
}





impl UniformSlice_1 for UniformData<&[f32]> {
    fn upload_uniform_slice_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 1 {
            gl.uniform1fv_with_f32_array(Some(loc), &self.0);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}

impl UniformSlice_1 for UniformData<&[i32]> {
    fn upload_uniform_slice_1(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 1 {
            let mut values:[i32;4] = [self.0[0], self.0[1], self.0[2], self.0[3]];
            gl.uniform1iv_with_i32_array(Some(loc), &mut values);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformSlice_2 for UniformData<&[f32]> {
    fn upload_uniform_slice_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 2 {
            gl.uniform2fv_with_f32_array(Some(loc), &self.0);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformSlice_2 for UniformData<&[i32]> {
    fn upload_uniform_slice_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 2 {
            let mut values:[i32;4] = [self.0[0], self.0[1], self.0[2], self.0[3]];
            gl.uniform2iv_with_i32_array(Some(loc), &mut values);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformSlice_3 for UniformData<&[f32]> {
    fn upload_uniform_slice_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 1 {
            gl.uniform3fv_with_f32_array(Some(loc), &self.0);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformSlice_3 for UniformData<&[i32]> {
    fn upload_uniform_slice_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 3 {
            let mut values:[i32;4] = [self.0[0], self.0[1], self.0[2], self.0[3]];
            gl.uniform3iv_with_i32_array(Some(loc), &mut values);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformSlice_4 for UniformData<&[f32]> {
    fn upload_uniform_slice_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 1 {
            gl.uniform4fv_with_f32_array(Some(loc), &self.0);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformSlice_4 for UniformData<&[i32]> {
    fn upload_uniform_slice_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 1 {
            let mut values:[i32;4] = [self.0[0], self.0[1], self.0[2], self.0[3]];
            gl.uniform4iv_with_i32_array(Some(loc), &mut values);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
/*
 * Uniform Matrix
 */
impl UniformMatrix for UniformData<[f32;4]> {
    fn upload_uniform_matrix(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform_matrix2fv_with_f32_array(Some(loc), false, &self.0);
    }
    fn upload_uniform_matrix_transposed(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform_matrix2fv_with_f32_array(Some(loc), true, &self.0);
    }
}
impl UniformMatrix_2 for UniformData<[f32;4]> {
    fn upload_uniform_matrix_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform_matrix2fv_with_f32_array(Some(loc), false, &self.0);
        Ok(())
    }
    fn upload_uniform_matrix_transposed_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform_matrix2fv_with_f32_array(Some(loc), true, &self.0);
        Ok(())
    }
}


impl UniformMatrix for UniformData<[f32;9]> {
    fn upload_uniform_matrix(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform_matrix3fv_with_f32_array(Some(loc), false, &self.0);
    }
    fn upload_uniform_matrix_transposed(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform_matrix3fv_with_f32_array(Some(loc), true, &self.0);
    }
}
impl UniformMatrix_3 for UniformData<[f32;9]> {
    fn upload_uniform_matrix_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform_matrix3fv_with_f32_array(Some(loc), false, &self.0);
        Ok(())
    }
    fn upload_uniform_matrix_transposed_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform_matrix3fv_with_f32_array(Some(loc), true, &self.0);
        Ok(())
    }
}

impl UniformMatrix for UniformData<[f32;16]> {
    fn upload_uniform_matrix(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform_matrix4fv_with_f32_array(Some(loc), false, &self.0);
    }
    fn upload_uniform_matrix_transposed(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) {
        gl.uniform_matrix4fv_with_f32_array(Some(loc), true, &self.0);
    }
}
impl UniformMatrix_4 for UniformData<[f32;16]> {
    fn upload_uniform_matrix_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform_matrix4fv_with_f32_array(Some(loc), false, &self.0);
        Ok(())
    }
    fn upload_uniform_matrix_transposed_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        gl.uniform_matrix4fv_with_f32_array(Some(loc), true, &self.0);
        Ok(())
    }
}


impl UniformMatrix_2 for UniformData<&[f32]> {
    fn upload_uniform_matrix_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 4 {
            gl.uniform_matrix2fv_with_f32_array(Some(loc), false, &self.0);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
    fn upload_uniform_matrix_transposed_2(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 4 {
            gl.uniform_matrix2fv_with_f32_array(Some(loc), true, &self.0);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}

impl UniformMatrix_3 for UniformData<&[f32]> {
    fn upload_uniform_matrix_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 9 {
            gl.uniform_matrix3fv_with_f32_array(Some(loc), false, &self.0);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
    fn upload_uniform_matrix_transposed_3(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 9 {
            gl.uniform_matrix3fv_with_f32_array(Some(loc), true, &self.0);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
impl UniformMatrix_4 for UniformData<&[f32]> {
    fn upload_uniform_matrix_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 16 {
            gl.uniform_matrix4fv_with_f32_array(Some(loc), false, &self.0);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
    fn upload_uniform_matrix_transposed_4(&self, gl:&WebGlContext, loc:&WebGlUniformLocation) -> Result<(), Error> {
        if (self.0).len() >= 16 {
            gl.uniform_matrix4fv_with_f32_array(Some(loc), true, &self.0);
            Ok(())
        } else {
            Err(Error::from(NativeError::UniformSliceSize))
        }
    }
}
//Renderer wrapper
impl WebGlRenderer {
    pub fn get_uniform_location_value(&self, name:&str) -> Result<WebGlUniformLocation, Error> {

        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        program_info.uniform_lookup
            .get(name)
            .map(|v| v.clone())
            .ok_or_else(|| Error::from(NativeError::UniformLocation(Some(name.to_string()))))
    }


    fn _get_uniform_loc(&self, target:&Uniform) -> Result<WebGlUniformLocation, Error> {
        match target {
            Uniform::Name(ref name) => {
                self.get_uniform_location_value(&name)
            },
            Uniform::Loc(ref loc) => {
                Ok(loc.clone())
            }
        }
    }

    pub fn upload_uniform_values<T: UniformValues>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_values(&self.gl, &loc);
        Ok(())
    }
    pub fn upload_uniform_values_1<T: UniformValues_1>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_values_1(&self.gl, &loc)
    }
    pub fn upload_uniform_values_2<T: UniformValues_2>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_values_2(&self.gl, &loc)
    }
    pub fn upload_uniform_values_3<T: UniformValues_3>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_values_3(&self.gl, &loc)
    }
    pub fn upload_uniform_values_4<T: UniformValues_4>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_values_4(&self.gl, &loc)
    }

    pub fn upload_uniform_slice<T: UniformSlice>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_slice(&self.gl, &loc);
        Ok(())
    }
    pub fn upload_uniform_slice_1<T: UniformSlice_1>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_slice_1(&self.gl, &loc)
    }
    pub fn upload_uniform_slice_2<T: UniformSlice_2>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_slice_2(&self.gl, &loc)
    }
    pub fn upload_uniform_slice_3<T: UniformSlice_3>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_slice_3(&self.gl, &loc)
    }
    pub fn upload_uniform_slice_4<T: UniformSlice_4>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_slice_4(&self.gl, &loc)
    }

    pub fn upload_uniform_matrix<T: UniformMatrix>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_matrix(&self.gl, &loc);
        Ok(())
    }
    pub fn upload_uniform_matrix_2<T: UniformMatrix_2>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_matrix_2(&self.gl, &loc)
    }
    pub fn upload_uniform_matrix_3<T: UniformMatrix_3>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_matrix_3(&self.gl, &loc)
    }
    pub fn upload_uniform_matrix_4<T: UniformMatrix_4>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_matrix_4(&self.gl, &loc)
    }

    pub fn upload_uniform_matrix_transposed<T: UniformMatrix>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_matrix_transposed(&self.gl, &loc);
        Ok(())
    }
    pub fn upload_uniform_matrix_transposed_2<T: UniformMatrix_2>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_matrix_transposed_2(&self.gl, &loc)
    }
    pub fn upload_uniform_matrix_transposed_3<T: UniformMatrix_3>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_matrix_transposed_3(&self.gl, &loc)
    }
    pub fn upload_uniform_matrix_transposed_4<T: UniformMatrix_4>(&self, target:&Uniform, data:&T) -> Result<(), Error> {
        let loc = self._get_uniform_loc(&target)?;
        data.upload_uniform_matrix_transposed_4(&self.gl, &loc)
    }
}
