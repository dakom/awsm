use crate::errors::{Error, NativeError};
use super::{WebGlRenderer, Id, BufferData, BufferTarget, BufferUsage};


impl WebGlRenderer {

    pub fn get_uniform_buffer_loc(&self, name:&str) -> Result<u32, Error> {

        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let loc = program_info.uniform_buffer_loc_lookup.get(name).ok_or(Error::from(NativeError::UniformBufferMissing(Some(name.to_string()))))?;

        Ok(*loc)
    }

    pub fn register_global_uniform_buffer(&mut self, name:&str) {
        self.ubo_global_loc_lookup.push(name.to_string());
    }

    pub fn get_global_uniform_buffer_loc(&self, name:&str) -> Option<u32> {
        self.ubo_global_loc_lookup
            .iter()
            .position(|global_name| name == *global_name)
            .map(|n| n as u32)
    }

    pub fn activate_uniform_buffer(&self, id:Id, name:&str) -> Result<(), Error> {
        let loc = self.get_uniform_buffer_loc(&name)?;
        self.bind_buffer_base(id, loc, BufferTarget::UniformBuffer)
    }
    pub fn upload_buffer_to_uniform_buffer_f32(&self, name:&str, id:Id, values:&[f32], buffer_usage:BufferUsage) -> Result<(), Error> {

        let buffer_data = BufferData::new(values, BufferTarget::UniformBuffer, buffer_usage);
        self.upload_buffer(id, buffer_data)?;
        self.activate_uniform_buffer(id, name)
    }
    
    pub fn upload_buffer_to_uniform_buffer_u8(&self, name:&str, id:Id, values:&[u8], buffer_usage:BufferUsage) -> Result<(), Error> {

        let buffer_data = BufferData::new(values, BufferTarget::UniformBuffer, buffer_usage);
        self.upload_buffer(id, buffer_data)?;
        self.activate_uniform_buffer(id, name)
    }
}
