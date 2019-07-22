use super::{BeginMode, DataType, WebGlRenderer};
use crate::errors::Error;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

impl WebGlRenderer<WebGlRenderingContext> {
    pub fn vertex_attrib_divisor(&self, loc: u32, divisor: u32) -> Result<(), Error> {
        let ext = self.get_extension_instanced_arrays()?;
        ext.vertex_attrib_divisor_angle(loc, divisor);
        Ok(())
    }

    pub fn draw_arrays_instanced(
        &self,
        mode: BeginMode,
        first: u32,
        count: u32,
        primcount: u32,
    ) -> Result<(), Error> {
        let ext = self.get_extension_instanced_arrays()?;
        ext.draw_arrays_instanced_angle(mode as u32, first as i32, count as i32, primcount as i32);
        Ok(())
    }

    pub fn draw_elements_instanced(
        &self,
        mode: BeginMode,
        count: u32,
        data_type: DataType,
        offset: u32,
        primcount: u32,
    ) -> Result<(), Error> {
        let ext = self.get_extension_instanced_arrays()?;
        ext.draw_elements_instanced_angle_with_i32(
            mode as u32,
            count as i32,
            data_type as u32,
            offset as i32,
            primcount as i32,
        );
        Ok(())
    }
}

impl WebGlRenderer<WebGl2RenderingContext> {
    pub fn vertex_attrib_divisor(&self, loc: u32, divisor: u32) -> Result<(), Error> {
        self.gl.vertex_attrib_divisor(loc, divisor);
        Ok(())
    }

    pub fn draw_arrays_instanced(
        &self,
        mode: BeginMode,
        first: u32,
        count: u32,
        primcount: u32,
    ) -> Result<(), Error> {
        self.gl
            .draw_arrays_instanced(mode as u32, first as i32, count as i32, primcount as i32);
        Ok(())
    }

    pub fn draw_elements_instanced(
        &self,
        mode: BeginMode,
        count: u32,
        data_type: DataType,
        offset: u32,
        primcount: u32,
    ) -> Result<(), Error> {
        self.gl.draw_elements_instanced_with_i32(
            mode as u32,
            count as i32,
            data_type as u32,
            offset as i32,
            primcount as i32,
        );
        Ok(())
    }
}
