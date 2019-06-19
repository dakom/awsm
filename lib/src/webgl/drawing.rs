use super::{WebGlContext, WebGlRenderer, ClearBufferMask, BeginMode, DataType};

pub fn clear_direct(gl:&WebGlContext, bits: &[ClearBufferMask]) {
    let mut combined = 0u32;
    for bit in bits {
        combined = combined | *bit as u32;
    }
    gl.clear(combined);
}

pub fn draw_arrays_direct(gl:&WebGlContext, mode: BeginMode, first: u32, count: u32) {
    gl.draw_arrays(mode as u32, first as i32, count as i32);
}

pub fn draw_elements_direct(gl:&WebGlContext, mode: BeginMode, count: u32, data_type:DataType, offset:u32) {
    gl.draw_elements_with_i32(mode as u32, count as i32, data_type as u32, offset as i32);
}

impl WebGlRenderer {
    pub fn clear(&self, bits: &[ClearBufferMask]) {
        clear_direct(&self.gl, &bits);
    }

    pub fn draw_arrays(&self, mode: BeginMode, first: u32, count: u32) {
        draw_arrays_direct(&self.gl, mode, first, count);
    }

    pub fn draw_elements(&self, mode: BeginMode, count: u32, data_type:DataType, offset:u32) {
        draw_elements_direct(&self.gl, mode, count, data_type, offset);
    }
}
