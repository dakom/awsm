use super::{
    BufferData, BufferDataImpl, BufferSubData, BufferSubDataImpl, BufferTarget, BufferUsage, Id, WebGlRenderer,
};
use crate::errors::{Error, NativeError};
use web_sys::WebGl2RenderingContext;

impl WebGlRenderer<WebGl2RenderingContext> {
    pub fn get_uniform_buffer_loc(&self, name: &str) -> Result<u32, Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self
            .program_lookup
            .get(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let loc = program_info
            .uniform_buffer_loc_lookup
            .get(name)
            .ok_or(Error::from(NativeError::UniformBufferMissing(Some(
                name.to_string(),
            ))))?;

        Ok(*loc)
    }

    pub fn get_uniform_block_offset(
        &self,
        uniform_name: &str,
        block_name: &str,
    ) -> Result<u32, Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self
            .program_lookup
            .get(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let offset_lookup = program_info
            .uniform_buffer_offset_lookup
            .get(block_name)
            .ok_or(Error::from(NativeError::UniformBufferMissing(Some(
                block_name.to_string(),
            ))))?;

        let offset = offset_lookup.get(uniform_name).ok_or(Error::from(
            NativeError::UniformBufferOffsetMissing(Some((
                block_name.to_string(),
                uniform_name.to_string(),
            ))),
        ))?;

        Ok(*offset)
    }

    pub fn register_global_uniform_buffer(&mut self, name: &str) {
        self.ubo_global_loc_lookup.push(name.to_string());
    }

    pub fn get_global_uniform_buffer_loc(&self, name: &str) -> Option<u32> {
        self.ubo_global_loc_lookup
            .iter()
            .position(|global_name| name == *global_name)
            .map(|n| n as u32)
    }

    pub fn activate_uniform_buffer(&self, id: Id, name: &str) -> Result<(), Error> {
        let loc = self.get_uniform_buffer_loc(&name)?;
        self.bind_buffer_base(id, loc, BufferTarget::UniformBuffer)
    }

    ///upload buffer data and set to uniform buffer
    pub fn upload_buffer_to_uniform_buffer<B: BufferDataImpl>(
        &self,
        name: &str,
        id: Id,
        buffer_data: B,
    ) -> Result<(), Error> {
        match buffer_data.get_target() {
            BufferTarget::UniformBuffer => {
                self.upload_buffer(id, buffer_data)?;
                self.activate_uniform_buffer(id, name)
            }
            _ => Err(Error::from(NativeError::UniformBufferTarget)),
        }
    }

    ///upload buffer data from sub slice and set to uniform buffer
    pub fn upload_buffer_sub_to_uniform_buffer<B: BufferSubDataImpl>(
        &self,
        uniform_name: &str,
        block_name: &str,
        id: Id,
        buffer_data: B,
    ) -> Result<(), Error> {
        match buffer_data.get_target() {
            BufferTarget::UniformBuffer => {
                let dest_byte_offset = self.get_uniform_block_offset(uniform_name, block_name)?;
                self.upload_buffer_sub(id, dest_byte_offset, buffer_data)?;
                self.activate_uniform_buffer(id, block_name)
            }
            _ => Err(Error::from(NativeError::UniformBufferTarget)),
        }
    }
    ///convenience function
    pub fn upload_buffer_to_uniform_buffer_f32(
        &self,
        name: &str,
        id: Id,
        values: &[f32],
        buffer_usage: BufferUsage,
    ) -> Result<(), Error> {
        self.upload_buffer_to_uniform_buffer(
            name,
            id,
            BufferData::new(values, BufferTarget::UniformBuffer, buffer_usage),
        )
    }

    ///convenience function
    pub fn upload_buffer_to_uniform_buffer_u8(
        &self,
        name: &str,
        id: Id,
        values: &[u8],
        buffer_usage: BufferUsage,
    ) -> Result<(), Error> {
        self.upload_buffer_to_uniform_buffer(
            name,
            id,
            BufferData::new(values, BufferTarget::UniformBuffer, buffer_usage),
        )
    }

    pub fn upload_buffer_sub_to_uniform_buffer_f32(
        &self,
        uniform_name: &str,
        block_name: &str,
        id: Id,
        values: &[f32],
    ) -> Result<(), Error> {
        self.upload_buffer_sub_to_uniform_buffer(
            uniform_name,
            block_name,
            id,
            BufferSubData::new(values, BufferTarget::UniformBuffer),
        )
    }

    pub fn upload_buffer_sub_to_uniform_buffer_u8(
        &self,
        uniform_name: &str,
        block_name: &str,
        id: Id,
        values: &[u8],
    ) -> Result<(), Error> {
        self.upload_buffer_sub_to_uniform_buffer(
            uniform_name,
            block_name,
            id,
            BufferSubData::new(values, BufferTarget::UniformBuffer),
        )
    }
}
