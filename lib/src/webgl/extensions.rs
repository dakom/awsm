use super::{WebGlRenderer};
use crate::errors::{Error, NativeError};

impl WebGlRenderer {
    pub fn register_extension(&mut self, name:&str) -> Result<&js_sys::Object, Error> {
        if self.extension_lookup.get(name).is_none() {
            let ext = self.gl.get_extension(name)?.ok_or(Error::from(NativeError::NoExtension))?;
            self.extension_lookup.insert(name.to_string(), ext); 
        }
        self.extension_lookup.get(name).ok_or(
            Error::from(NativeError::NoExtension)
        )
    }

    pub(super) fn get_extension(&self, name:&str) -> Result<&js_sys::Object, Error> {
        self.extension_lookup.get(name).ok_or(
            Error::from(NativeError::NoExtension)
        )
    }
}


