use crate::errors::{Error, NativeError};
use wasm_bindgen::{JsCast};
use web_sys::{WebGlVertexArrayObject};
use super::{WebGlRenderer, Id, AttributeOptions, BufferTarget, BufferDataImpl};


pub struct UniformBuffer {
    pub block_index: u32,
    pub bind_point: u32
}

impl WebGlRenderer {

    pub fn get_uniform_buffer(&self, name:&str) -> Result<&UniformBuffer, Error> {

        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        program_info.uniform_buffer_lookup.get(name).ok_or(Error::from(NativeError::UniformBufferMissing(Some(name.to_string()))))
    }

    pub fn bind_uniform_buffer(&self, name:&str) -> Result<(), Error> {
        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let uniform_buffer = self.get_uniform_buffer(&name)?;
        self.gl.uniform_block_binding(&program_info.program, uniform_buffer.block_index, uniform_buffer.bind_point);
        Ok(())
    }

    pub fn upload_buffer_to_uniform_buffer<T: BufferDataImpl>(&self, id:Id, data:T, name:&str) -> Result<(), Error> {
        let target = data.get_target();
        let uniform_buffer = self.get_uniform_buffer(&name)?;
        self.upload_buffer(id, data)?;
        self.bind_uniform_buffer(&name)?;
        self.bind_buffer_base(id, uniform_buffer.block_index, target)
    }
}
