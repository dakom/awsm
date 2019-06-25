use crate::errors::{Error, NativeError};
use wasm_bindgen::{JsCast};
use web_sys::{WebGlVertexArrayObject};
use super::{WebGlRenderer, Id, AttributeOptions, BufferTarget};

use cfg_if::cfg_if;

pub struct VertexArray<'a> {
    pub attribute_name: &'a str, 
    pub buffer_id: Id,
    pub opts: &'a AttributeOptions
}

impl <'a> VertexArray<'a> {
    pub fn new(attribute_name:&'a str, buffer_id: Id, opts: &'a AttributeOptions) -> Self {
        Self {
            attribute_name,
            buffer_id,
            opts
        }
    }
}

cfg_if! {
    if #[cfg(feature = "webgl_1")] {
        use web_sys::{OesVertexArrayObject};


        pub fn bind_vertex_array_direct(ext:&OesVertexArrayObject, vao:Option<&WebGlVertexArrayObject>) {
            ext.bind_vertex_array_oes(vao);
        }
        pub fn create_vertex_array_direct(ext:&OesVertexArrayObject) -> Result<WebGlVertexArrayObject, Error> {
            ext.create_vertex_array_oes().ok_or(Error::from(NativeError::VertexArrayCreate))
        }

        impl WebGlRenderer {
            pub fn register_extension_vertex_array(&mut self) -> Result<&OesVertexArrayObject, Error> {
                self.register_extension("OES_vertex_array_object")
                    .map(|ext| ext.unchecked_ref::<OesVertexArrayObject>())
            }
            fn _get_extension_vertex_array(&self) -> Result<&OesVertexArrayObject, Error> {
                self.get_extension("OES_vertex_array_object")
                    .map(|ext| ext.unchecked_ref::<OesVertexArrayObject>())
            }

            fn _bind_vertex_array(&self, id:Option<Id>, vao:Option<&WebGlVertexArrayObject>) -> Result<(), Error> {
                let ext = self._get_extension_vertex_array()?;
                bind_vertex_array_direct(&ext, vao);
                self.current_vao_id.set(id);
                Ok(())
            }

            pub fn create_vertex_array(&mut self) -> Result<Id, Error> {
                let ext = self._get_extension_vertex_array()?;
                let vao = create_vertex_array_direct(&ext)?;
                let id = self.vao_lookup.insert(vao);
                Ok(id)
            }
        }

    } else if #[cfg(feature = "webgl_2")] {
        use super::{WebGlContext};

        pub fn bind_vertex_array_direct(gl:&WebGlContext, vao:Option<&WebGlVertexArrayObject>) {
            gl.bind_vertex_array(vao);
        }
        pub fn create_vertex_array_direct(gl:&WebGlContext) -> Result<WebGlVertexArrayObject, Error> {
            gl.create_vertex_array().ok_or(Error::from(NativeError::VertexArrayCreate))
        }
        impl WebGlRenderer {
            fn _bind_vertex_array(&self, id:Option<Id>, vao:Option<&WebGlVertexArrayObject>) -> Result<(), Error> {
                bind_vertex_array_direct(&self.gl, vao);
                self.current_vao_id.set(id);
                Ok(())
            }

            pub fn create_vertex_array(&mut self) -> Result<Id, Error> {
                let vao = create_vertex_array_direct(&self.gl)?;
                let id = self.vao_lookup.insert(vao);
                Ok(id)
            }
        }
    }
}


impl WebGlRenderer {

    pub fn release_vertex_array(&self) -> Result<(), Error> {
        self._bind_vertex_array(None, None)
    }

    pub fn activate_vertex_array(&self, vao_id:Id) -> Result<(), Error> {
        if Some(vao_id) != self.current_vao_id.get() {
            if let Some(vao) = self.vao_lookup.get(vao_id) { 
                self._bind_vertex_array(Some(vao_id), Some(&vao));
            } else {
                return Err(Error::from(NativeError::VertexArrayMissing));
            }
        }
        Ok(())
    }

    pub fn assign_vertex_array(&self, vao_id:Id, element_buffer_id:Option<Id>, configs:&[VertexArray]) -> Result<(), Error> {
        let result = if let Some(vao) = self.vao_lookup.get(vao_id) { 
            self._bind_vertex_array(Some(vao_id), Some(&vao));

            //Skip buffer assignment cache checks
            if let Some(element_buffer_id) = element_buffer_id {
                self._bind_buffer_nocheck(element_buffer_id, BufferTarget::ElementArrayBuffer)?;
            }

            for config in configs {
                self._bind_buffer_nocheck(config.buffer_id, BufferTarget::ArrayBuffer)?;
                self.activate_attribute(&config.attribute_name, &config.opts)?;
            }
            Ok(())
        } else {
            Err(Error::from(NativeError::VertexArrayMissing))
        };
           
        //relase it for the next call that might use elements
        self.release_vertex_array();

        result
    }
    
}
