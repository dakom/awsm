use super::{WebGlCommon, WebGlRenderer};
use crate::errors::{Error, NativeError};
use std::marker::PhantomData;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};
use web_sys::{WebGlProgram, WebGlUniformLocation};

pub enum UniformType {
    Scalar1,
    Scalar2,
    Scalar3,
    Scalar4,
    Vector1,
    Vector2,
    Vector3,
    Vector4,
    Matrix2,
    Matrix3,
    Matrix4,
    MatrixTransposed2,
    MatrixTransposed3,
    MatrixTransposed4,
}

pub trait PartialWebGlUniforms {
    fn awsm_get_uniform_location(
        &self,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<WebGlUniformLocation, Error>;
    fn awsm_upload_uniform_fvec<T: AsRef<[f32]>>(
        &self,
        loc: &WebGlUniformLocation,
        _type: UniformType,
        data: &T,
    ) -> Result<(), Error>;
    fn awsm_upload_uniform_ivec<T: AsRef<[i32]>>(
        &self,
        loc: &WebGlUniformLocation,
        _type: UniformType,
        data: &T,
    ) -> Result<(), Error>;

    fn awsm_uniform1f(&self, loc: &WebGlUniformLocation, x: f32);
    fn awsm_uniform2f(&self, loc: &WebGlUniformLocation, x: f32, y: f32);
    fn awsm_uniform3f(&self, loc: &WebGlUniformLocation, x: f32, y: f32, z: f32);
    fn awsm_uniform4f(&self, loc: &WebGlUniformLocation, x: f32, y: f32, z: f32, w: f32);

    fn awsm_uniform1fv_with_f32_array(&self, loc: &WebGlUniformLocation, data: &[f32]);
    fn awsm_uniform2fv_with_f32_array(&self, loc: &WebGlUniformLocation, data: &[f32]);
    fn awsm_uniform3fv_with_f32_array(&self, loc: &WebGlUniformLocation, data: &[f32]);
    fn awsm_uniform4fv_with_f32_array(&self, loc: &WebGlUniformLocation, data: &[f32]);

    fn awsm_uniform1i(&self, loc: &WebGlUniformLocation, x: i32);
    fn awsm_uniform2i(&self, loc: &WebGlUniformLocation, x: i32, y: i32);
    fn awsm_uniform3i(&self, loc: &WebGlUniformLocation, x: i32, y: i32, z: i32);
    fn awsm_uniform4i(&self, loc: &WebGlUniformLocation, x: i32, y: i32, z: i32, w: i32);

    fn awsm_uniform1iv_with_i32_array(&self, loc: &WebGlUniformLocation, data: &[i32]);
    fn awsm_uniform2iv_with_i32_array(&self, loc: &WebGlUniformLocation, data: &[i32]);
    fn awsm_uniform3iv_with_i32_array(&self, loc: &WebGlUniformLocation, data: &[i32]);
    fn awsm_uniform4iv_with_i32_array(&self, loc: &WebGlUniformLocation, data: &[i32]);

    fn awsm_uniform_matrix2fv_with_f32_array(
        &self,
        loc: &WebGlUniformLocation,
        transpose: bool,
        data: &[f32],
    );
    fn awsm_uniform_matrix3fv_with_f32_array(
        &self,
        loc: &WebGlUniformLocation,
        transpose: bool,
        data: &[f32],
    );
    fn awsm_uniform_matrix4fv_with_f32_array(
        &self,
        loc: &WebGlUniformLocation,
        transpose: bool,
        data: &[f32],
    );
}

pub trait PartialWebGl2Uniforms {
    fn awsm_upload_uniform_uvec<T: AsRef<[u32]>>(
        &self,
        loc: &WebGlUniformLocation,
        _type: UniformType,
        data: &T,
    ) -> Result<(), Error>;
    fn awsm_uniform1ui(&self, loc: &WebGlUniformLocation, x: u32);
    fn awsm_uniform2ui(&self, loc: &WebGlUniformLocation, x: u32, y: u32);
    fn awsm_uniform3ui(&self, loc: &WebGlUniformLocation, x: u32, y: u32, z: u32);
    fn awsm_uniform4ui(&self, loc: &WebGlUniformLocation, x: u32, y: u32, z: u32, w: u32);

    fn awsm_uniform1uiv_with_u32_array(&self, loc: &WebGlUniformLocation, data: &[u32]);
    fn awsm_uniform2uiv_with_u32_array(&self, loc: &WebGlUniformLocation, data: &[u32]);
    fn awsm_uniform3uiv_with_u32_array(&self, loc: &WebGlUniformLocation, data: &[u32]);
    fn awsm_uniform4uiv_with_u32_array(&self, loc: &WebGlUniformLocation, data: &[u32]);
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlUniforms for $type {

            fn awsm_get_uniform_location(&self, program:&WebGlProgram, name:&str) -> Result<WebGlUniformLocation, Error> {
                self.get_uniform_location(&program, &name)
                    .ok_or(Error::from(NativeError::UniformLocation(Some(name.to_owned()))))
            }

            fn awsm_upload_uniform_fvec<T: AsRef<[f32]>> (&self, loc:&WebGlUniformLocation, _type:UniformType, data:&T) -> Result<(), Error> {
                UniformSlice::new(data, _type).upload(self, &loc)
            }

            fn awsm_upload_uniform_ivec<T: AsRef<[i32]>> (&self, loc:&WebGlUniformLocation, _type:UniformType, data:&T) -> Result<(), Error> {
                UniformSlice::new(data, _type).upload(self, &loc)
            }


            fn awsm_uniform1f(&self, loc: &WebGlUniformLocation, x: f32) {
                self.uniform1f(Some(loc), x)
            }
            fn awsm_uniform2f(&self, loc: &WebGlUniformLocation, x: f32, y: f32) {
                self.uniform2f(Some(loc), x, y)
            }
            fn awsm_uniform3f(&self, loc: &WebGlUniformLocation, x: f32, y: f32, z: f32) {
                self.uniform3f(Some(loc), x, y, z)
            }
            fn awsm_uniform4f(&self, loc: &WebGlUniformLocation, x: f32, y: f32, z: f32, w: f32) {
                self.uniform4f(Some(loc), x, y, z, w)
            }

            fn awsm_uniform1fv_with_f32_array(&self, loc: &WebGlUniformLocation, data: &[f32]) {
                self.uniform1fv_with_f32_array(Some(loc), data)
            }
            fn awsm_uniform2fv_with_f32_array(&self, loc: &WebGlUniformLocation, data: &[f32]) {
                self.uniform2fv_with_f32_array(Some(loc), data)
            }
            fn awsm_uniform3fv_with_f32_array(&self, loc: &WebGlUniformLocation, data: &[f32]) {
                self.uniform3fv_with_f32_array(Some(loc), data)
            }
            fn awsm_uniform4fv_with_f32_array(&self, loc: &WebGlUniformLocation, data: &[f32]) {
                self.uniform4fv_with_f32_array(Some(loc), data)
            }


            fn awsm_uniform1i(&self, loc: &WebGlUniformLocation, x: i32) {
                self.uniform1i(Some(loc), x)
            }
            fn awsm_uniform2i(&self, loc: &WebGlUniformLocation, x: i32, y: i32) {
                self.uniform2i(Some(loc), x, y)
            }
            fn awsm_uniform3i(&self, loc: &WebGlUniformLocation, x: i32, y: i32, z: i32) {
                self.uniform3i(Some(loc), x, y, z)
            }
            fn awsm_uniform4i(&self, loc: &WebGlUniformLocation, x: i32, y: i32, z: i32, w: i32) {
                self.uniform4i(Some(loc), x, y, z, w)
            }

             /* TODO - followup with https://github.com/rustwasm/wasm-bindgen/pull/1539
             * When the i32 slices don't need mut anymore - get rid of making the mut clone
             */
            fn awsm_uniform1iv_with_i32_array(&self, loc: &WebGlUniformLocation, data: &[i32]) {
                self.uniform1iv_with_i32_array(Some(loc), data)
            }
            fn awsm_uniform2iv_with_i32_array(&self, loc: &WebGlUniformLocation, data: &[i32]) {
                self.uniform2iv_with_i32_array(Some(loc), data)
            }
            fn awsm_uniform3iv_with_i32_array(&self, loc: &WebGlUniformLocation, data: &[i32]) {
                self.uniform3iv_with_i32_array(Some(loc), data)
            }
            fn awsm_uniform4iv_with_i32_array(&self, loc: &WebGlUniformLocation, data: &[i32]) {
                self.uniform4iv_with_i32_array(Some(loc), data)
            }


            fn awsm_uniform_matrix2fv_with_f32_array(&self, loc: &WebGlUniformLocation, transpose: bool, data: &[f32]) {
                self.uniform_matrix2fv_with_f32_array(Some(loc), transpose, data);
            }
            fn awsm_uniform_matrix3fv_with_f32_array(&self, loc: &WebGlUniformLocation, transpose: bool, data: &[f32]) {
                self.uniform_matrix3fv_with_f32_array(Some(loc), transpose, data);
            }
            fn awsm_uniform_matrix4fv_with_f32_array(&self, loc: &WebGlUniformLocation, transpose: bool, data: &[f32]) {
                self.uniform_matrix4fv_with_f32_array(Some(loc), transpose, data);
            }
            $($defs)*
        })+
    };
}

impl_context! {
    WebGlRenderingContext{}
    WebGl2RenderingContext{}
}

impl PartialWebGl2Uniforms for WebGl2RenderingContext {
    fn awsm_upload_uniform_uvec<T: AsRef<[u32]>>(
        &self,
        loc: &WebGlUniformLocation,
        _type: UniformType,
        data: &T,
    ) -> Result<(), Error> {
        UniformSlice::new(data, _type).upload(self, &loc)
    }

    fn awsm_uniform1ui(&self, loc: &WebGlUniformLocation, x: u32) {
        self.uniform1ui(Some(loc), x)
    }
    fn awsm_uniform2ui(&self, loc: &WebGlUniformLocation, x: u32, y: u32) {
        self.uniform2ui(Some(loc), x, y)
    }
    fn awsm_uniform3ui(&self, loc: &WebGlUniformLocation, x: u32, y: u32, z: u32) {
        self.uniform3ui(Some(loc), x, y, z)
    }
    fn awsm_uniform4ui(&self, loc: &WebGlUniformLocation, x: u32, y: u32, z: u32, w: u32) {
        self.uniform4ui(Some(loc), x, y, z, w)
    }

    /* TODO - followup with https://github.com/rustwasm/wasm-bindgen/pull/1639
     * When the u32 slices don't need mut anymore - get rid of making the mut clone
     */
    fn awsm_uniform1uiv_with_u32_array(&self, loc: &WebGlUniformLocation, data: &[u32]) {
        let mut values = data.to_owned();
        self.uniform1uiv_with_u32_array(Some(loc), &mut values)
    }
    fn awsm_uniform2uiv_with_u32_array(&self, loc: &WebGlUniformLocation, data: &[u32]) {
        let mut values = data.to_owned();
        self.uniform2uiv_with_u32_array(Some(loc), &mut values)
    }
    fn awsm_uniform3uiv_with_u32_array(&self, loc: &WebGlUniformLocation, data: &[u32]) {
        let mut values = data.to_owned();
        self.uniform3uiv_with_u32_array(Some(loc), &mut values)
    }
    fn awsm_uniform4uiv_with_u32_array(&self, loc: &WebGlUniformLocation, data: &[u32]) {
        let mut values = data.to_owned();
        self.uniform4uiv_with_u32_array(Some(loc), &mut values)
    }
}
/*
 * The slice-based uploads are written as traits on this a newtype wrapper
 * in order to work with either f32 or i32 and still get simple checks
 *
 * There is no need to wrap the scalar versions because the only check
 * for those is the length, which is known at compile-time
 *
 * Realistically, the renderer's convenience functions provide more value
 * since they expand on this to also get the location by name and provide more wrappers
 *
 * Technique via https://users.rust-lang.org/t/different-impls-for-types-of-slices-and-arrays/29468
 * Playground proof-of-concept: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=19dc3ce889d1e7c8aedbd36ad45b8422
 * TODO
 * 1. followup with https://github.com/rustwasm/wasm-bindgen/pull/1539
 * When the i32 slices don't need mut anymore - simplify below
 *
 *
 */

pub struct UniformSlice<T, U> {
    values: T,
    _type: UniformType,
    phantom: PhantomData<U>,
}

impl<T: AsRef<[U]>, U> UniformSlice<T, U> {
    pub fn new(values: T, _type: UniformType) -> Self {
        Self {
            values,
            _type,
            phantom: PhantomData,
        }
    }
}

pub trait UniformUploadImpl {
    fn upload<T: PartialWebGlUniforms>(
        &self,
        gl: &T,
        loc: &WebGlUniformLocation,
    ) -> Result<(), Error>;
}

pub trait UniformUploadImpl2 {
    fn upload<T: PartialWebGl2Uniforms>(
        &self,
        gl: &T,
        loc: &WebGlUniformLocation,
    ) -> Result<(), Error>;
}

fn is_length_enough(len: usize, _type: &UniformType) -> Result<(), Error> {
    let min_length = match _type {
        UniformType::Scalar1 | UniformType::Vector1 => 1,
        UniformType::Scalar2 | UniformType::Vector2 => 2,
        UniformType::Scalar3 | UniformType::Vector3 => 3,
        UniformType::Scalar4
        | UniformType::Vector4
        | UniformType::Matrix2
        | UniformType::MatrixTransposed2 => 4,
        UniformType::Matrix3 | UniformType::MatrixTransposed3 => 9,
        UniformType::Matrix4 | UniformType::MatrixTransposed4 => 16,
    };

    if len >= min_length {
        Ok(())
    } else {
        Err(Error::from(NativeError::UniformSize))
    }
}

impl<T: AsRef<[f32]>> UniformUploadImpl for UniformSlice<T, f32> {
    fn upload<G: PartialWebGlUniforms>(
        &self,
        gl: &G,
        loc: &WebGlUniformLocation,
    ) -> Result<(), Error> {
        let values = self.values.as_ref();
        is_length_enough(values.len(), &self._type)?;

        match self._type {
            UniformType::Scalar1 => gl.awsm_uniform1f(loc, values[0]),
            UniformType::Scalar2 => gl.awsm_uniform2f(loc, values[0], values[1]),
            UniformType::Scalar3 => gl.awsm_uniform3f(loc, values[0], values[1], values[2]),
            UniformType::Scalar4 => {
                gl.awsm_uniform4f(loc, values[0], values[1], values[2], values[3])
            }

            UniformType::Vector1 => gl.awsm_uniform1fv_with_f32_array(loc, values),
            UniformType::Vector2 => gl.awsm_uniform2fv_with_f32_array(loc, values),
            UniformType::Vector3 => gl.awsm_uniform3fv_with_f32_array(loc, values),
            UniformType::Vector4 => gl.awsm_uniform4fv_with_f32_array(loc, values),

            UniformType::Matrix2 => gl.awsm_uniform_matrix2fv_with_f32_array(loc, false, values),
            UniformType::Matrix3 => gl.awsm_uniform_matrix3fv_with_f32_array(loc, false, values),
            UniformType::Matrix4 => gl.awsm_uniform_matrix4fv_with_f32_array(loc, false, values),

            UniformType::MatrixTransposed2 => {
                gl.awsm_uniform_matrix2fv_with_f32_array(loc, true, values)
            }
            UniformType::MatrixTransposed3 => {
                gl.awsm_uniform_matrix3fv_with_f32_array(loc, true, values)
            }
            UniformType::MatrixTransposed4 => {
                gl.awsm_uniform_matrix4fv_with_f32_array(loc, true, values)
            }
        };

        Ok(())
    }
}

impl<T: AsRef<[i32]>> UniformUploadImpl for UniformSlice<T, i32> {
    fn upload<G: PartialWebGlUniforms>(
        &self,
        gl: &G,
        loc: &WebGlUniformLocation,
    ) -> Result<(), Error> {
        let values = self.values.as_ref();
        is_length_enough(values.len(), &self._type)?;

        match self._type {
            UniformType::Scalar1 => gl.awsm_uniform1i(loc, values[0]),
            UniformType::Scalar2 => gl.awsm_uniform2i(loc, values[0], values[1]),
            UniformType::Scalar3 => gl.awsm_uniform3i(loc, values[0], values[1], values[2]),
            UniformType::Scalar4 => {
                gl.awsm_uniform4i(loc, values[0], values[1], values[2], values[3])
            }

            UniformType::Vector1 => gl.awsm_uniform1iv_with_i32_array(loc, values),
            UniformType::Vector2 => gl.awsm_uniform2iv_with_i32_array(loc, values),
            UniformType::Vector3 => gl.awsm_uniform3iv_with_i32_array(loc, values),
            UniformType::Vector4 => gl.awsm_uniform4iv_with_i32_array(loc, values),

            _ => return Err(Error::from(NativeError::UniformMatrixMustBeFloat)),
        };

        Ok(())
    }
}

impl<T: AsRef<[u32]>> UniformUploadImpl2 for UniformSlice<T, u32> {
    fn upload<G: PartialWebGl2Uniforms>(
        &self,
        gl: &G,
        loc: &WebGlUniformLocation,
    ) -> Result<(), Error> {
        let values = self.values.as_ref();
        is_length_enough(values.len(), &self._type)?;

        match self._type {
            UniformType::Scalar1 => gl.awsm_uniform1ui(loc, values[0]),
            UniformType::Scalar2 => gl.awsm_uniform2ui(loc, values[0], values[1]),
            UniformType::Scalar3 => gl.awsm_uniform3ui(loc, values[0], values[1], values[2]),
            UniformType::Scalar4 => {
                gl.awsm_uniform4ui(loc, values[0], values[1], values[2], values[3])
            }

            UniformType::Vector1 => gl.awsm_uniform1uiv_with_u32_array(loc, values),
            UniformType::Vector2 => gl.awsm_uniform2uiv_with_u32_array(loc, values),
            UniformType::Vector3 => gl.awsm_uniform3uiv_with_u32_array(loc, values),
            UniformType::Vector4 => gl.awsm_uniform4uiv_with_u32_array(loc, values),

            _ => return Err(Error::from(NativeError::UniformMatrixMustBeFloat)),
        };

        Ok(())
    }
}

//Renderer wrapper
//The uniform lookups are cached at shader compilation (see shader.rs)
impl<G: WebGlCommon> WebGlRenderer<G> {
    pub fn get_uniform_location_value(&self, name: &str) -> Result<WebGlUniformLocation, Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self
            .program_lookup
            .get(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        program_info
            .uniform_lookup
            .get(name)
            .map(|v| v.clone())
            .ok_or_else(|| Error::from(NativeError::UniformLocation(Some(name.to_string()))))
    }

    //this covers all the slice-based versions due to the impl above

    //Just some convenience helpers
    pub fn upload_uniform_fvec<T: AsRef<[f32]>>(
        &self,
        target_name: &str,
        _type: UniformType,
        data: &T,
    ) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_upload_uniform_fvec(&loc, _type, data)
    }

    pub fn upload_uniform_ivec<T: AsRef<[i32]>>(
        &self,
        target_name: &str,
        _type: UniformType,
        data: &T,
    ) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_upload_uniform_ivec(&loc, _type, data)
    }

    pub fn upload_uniform_mat_4<T: AsRef<[f32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_fvec(target_name, UniformType::Matrix4, &data)
    }
    pub fn upload_uniform_mat_3<T: AsRef<[f32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_fvec(target_name, UniformType::Matrix3, data)
    }
    pub fn upload_uniform_mat_2<T: AsRef<[f32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_fvec(target_name, UniformType::Matrix2, data)
    }
    pub fn upload_uniform_mat_transposed_4<T: AsRef<[f32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_fvec(target_name, UniformType::MatrixTransposed4, data)
    }
    pub fn upload_uniform_mat_transposed_3<T: AsRef<[f32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_fvec(target_name, UniformType::MatrixTransposed3, data)
    }
    pub fn upload_uniform_mat_transposed_2<T: AsRef<[f32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_fvec(target_name, UniformType::MatrixTransposed2, data)
    }

    pub fn upload_uniform_fvec_4<T: AsRef<[f32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_fvec(target_name, UniformType::Vector4, data)
    }
    pub fn upload_uniform_fvec_3<T: AsRef<[f32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_fvec(target_name, UniformType::Vector3, data)
    }
    pub fn upload_uniform_fvec_2<T: AsRef<[f32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_fvec(target_name, UniformType::Vector2, data)
    }
    pub fn upload_uniform_fvec_1<T: AsRef<[f32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_fvec(target_name, UniformType::Vector1, data)
    }

    pub fn upload_uniform_ivec_4<T: AsRef<[i32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_ivec(target_name, UniformType::Vector4, data)
    }
    pub fn upload_uniform_ivec_3<T: AsRef<[i32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_ivec(target_name, UniformType::Vector3, data)
    }
    pub fn upload_uniform_ivec_2<T: AsRef<[i32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_ivec(target_name, UniformType::Vector2, data)
    }
    pub fn upload_uniform_ivec_1<T: AsRef<[i32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_ivec(target_name, UniformType::Vector1, data)
    }

    //Scalar versions - only need "convenience" form with string because if the caller
    //already knows the location, there's no reason to just use the context directly

    pub fn upload_uniform_fvals_4(
        &self,
        target_name: &str,
        data: (f32, f32, f32, f32),
    ) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_uniform4f(&loc, data.0, data.1, data.2, data.3);
        Ok(())
    }
    pub fn upload_uniform_fvals_3(
        &self,
        target_name: &str,
        data: (f32, f32, f32),
    ) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_uniform3f(&loc, data.0, data.1, data.2);
        Ok(())
    }
    pub fn upload_uniform_fvals_2(&self, target_name: &str, data: (f32, f32)) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_uniform2f(&loc, data.0, data.1);
        Ok(())
    }
    pub fn upload_uniform_fval(&self, target_name: &str, data: f32) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_uniform1f(&loc, data);
        Ok(())
    }

    pub fn upload_uniform_ivals_4(
        &self,
        target_name: &str,
        data: (i32, i32, i32, i32),
    ) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_uniform4i(&loc, data.0, data.1, data.2, data.3);
        Ok(())
    }
    pub fn upload_uniform_ivals_3(
        &self,
        target_name: &str,
        data: (i32, i32, i32),
    ) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_uniform3i(&loc, data.0, data.1, data.2);
        Ok(())
    }
    pub fn upload_uniform_ivals_2(&self, target_name: &str, data: (i32, i32)) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_uniform2i(&loc, data.0, data.1);
        Ok(())
    }
    pub fn upload_uniform_ival(&self, target_name: &str, data: i32) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_uniform1i(&loc, data);
        Ok(())
    }
}

impl WebGlRenderer<WebGl2RenderingContext> {
    pub fn upload_uniform_uvec<T: AsRef<[u32]>>(
        &self,
        target_name: &str,
        _type: UniformType,
        data: &T,
    ) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_upload_uniform_uvec(&loc, _type, data)
    }
    pub fn upload_uniform_uvec_4<T: AsRef<[u32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_uvec(target_name, UniformType::Vector4, data)
    }
    pub fn upload_uniform_uvec_3<T: AsRef<[u32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_uvec(target_name, UniformType::Vector3, data)
    }
    pub fn upload_uniform_uvec_2<T: AsRef<[u32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_uvec(target_name, UniformType::Vector2, data)
    }
    pub fn upload_uniform_uvec_1<T: AsRef<[u32]>>(
        &self,
        target_name: &str,
        data: &T,
    ) -> Result<(), Error> {
        self.upload_uniform_uvec(target_name, UniformType::Vector1, data)
    }

    pub fn upload_uniform_uvals_4(
        &self,
        target_name: &str,
        data: (u32, u32, u32, u32),
    ) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl
            .awsm_uniform4ui(&loc, data.0, data.1, data.2, data.3);
        Ok(())
    }
    pub fn upload_uniform_uvals_3(
        &self,
        target_name: &str,
        data: (u32, u32, u32),
    ) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_uniform3ui(&loc, data.0, data.1, data.2);
        Ok(())
    }
    pub fn upload_uniform_uvals_2(&self, target_name: &str, data: (u32, u32)) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_uniform2ui(&loc, data.0, data.1);
        Ok(())
    }
    pub fn upload_uniform_uval(&self, target_name: &str, data: u32) -> Result<(), Error> {
        let loc = self.get_uniform_location_value(&target_name)?;
        self.gl.awsm_uniform1ui(&loc, data);
        Ok(())
    }
}
