use super::{WebGlCommon, WebGlRenderer};
use crate::errors::{Error, NativeError};
use wasm_bindgen::JsCast;
use web_sys::AngleInstancedArrays;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

pub trait PartialWebGlExtensions {
    fn awsm_get_extension(&self, name: &str) -> Result<js_sys::Object, Error>;
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlExtensions for $type {
            fn awsm_get_extension(&self, name:&str) -> Result<js_sys::Object, Error> {
                self.get_extension(name)?.ok_or(Error::from(NativeError::NoExtension))
            }
            $($defs)*
        })+
    };
}

impl_context! {
    WebGlRenderingContext{}
    WebGl2RenderingContext{}
}

impl<T: WebGlCommon> WebGlRenderer<T> {
    pub fn register_extension(&mut self, name: &str) -> Result<&js_sys::Object, Error> {
        if self.extension_lookup.get(name).is_none() {
            let ext = self.gl.awsm_get_extension(name)?;
            self.extension_lookup.insert(name.to_string(), ext);
        }
        self.get_extension(name)
    }

    pub(super) fn get_extension(&self, name: &str) -> Result<&js_sys::Object, Error> {
        self.extension_lookup
            .get(name)
            .ok_or(Error::from(NativeError::NoExtension))
    }
}

impl WebGlRenderer<WebGlRenderingContext> {
    pub fn register_extension_instanced_arrays(&mut self) -> Result<&AngleInstancedArrays, Error> {
        self.register_extension("ANGLE_instanced_arrays")
            .map(|ext| ext.unchecked_ref::<AngleInstancedArrays>())
    }
    pub fn get_extension_instanced_arrays(&self) -> Result<&AngleInstancedArrays, Error> {
        self.get_extension("ANGLE_instanced_arrays")
            .map(|ext| ext.unchecked_ref::<AngleInstancedArrays>())
    }
}
