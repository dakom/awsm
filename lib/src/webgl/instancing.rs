use super::{WebGlRenderer, DataType, BeginMode};
use wasm_bindgen::JsCast;
use crate::errors::{Error};
use cfg_if::cfg_if;


cfg_if! {
    if #[cfg(feature = "webgl_1")] {
        use web_sys::{AngleInstancedArrays};

        pub fn vertex_attrib_divisor_direct(ext:&AngleInstancedArrays, loc:u32, divisor:u32) {
            ext.vertex_attrib_divisor_angle(loc, divisor);
        }


        pub fn draw_arrays_instanced_direct(ext:&AngleInstancedArrays, mode: BeginMode, first: u32, count: u32, primcount: u32) {
                ext.draw_arrays_instanced_angle(mode as u32, first as i32, count as i32, primcount as i32);
        }
        pub fn draw_elements_instanced_direct(ext:&AngleInstancedArrays, mode: BeginMode, count: u32, data_type: DataType, offset: u32, primcount: u32) {
                ext.draw_elements_instanced_angle_with_i32(mode as u32, count as i32, data_type as u32, offset as i32, primcount as i32);
        }

        impl WebGlRenderer {
            pub fn register_extension_instanced_arrays(&mut self) -> Result<&AngleInstancedArrays, Error> {
                self.register_extension("ANGLE_instanced_arrays")
                    .map(|ext| ext.unchecked_ref::<AngleInstancedArrays>())
            }
            pub fn get_extension_instanced_arrays(&self) -> Result<&AngleInstancedArrays, Error> {
                self.get_extension("ANGLE_instanced_arrays")
                    .map(|ext| ext.unchecked_ref::<AngleInstancedArrays>())
            }

            pub fn vertex_attrib_divisor(&self, loc: u32, divisor: u32) -> Result<(), Error> {
                let ext = self.get_extension_instanced_arrays()?;
                vertex_attrib_divisor_direct(&ext, loc, divisor);
                Ok(())
            }

            pub fn draw_arrays_instanced(&self, mode: BeginMode, first: u32, count: u32, primcount: u32) -> Result<(), Error> {
                let ext = self.get_extension_instanced_arrays()?;
                draw_arrays_instanced_direct(&ext, mode, first, count, primcount);
                Ok(())
            }

            pub fn draw_elements_instanced(&self, mode: BeginMode, count: u32, data_type: DataType, offset: u32, primcount: u32) -> Result<(), Error> {
                let ext = self.get_extension_instanced_arrays()?;
                draw_elements_instanced_direct(&ext, mode, count, data_type, offset, primcount);
                Ok(())
            }
        }
    } else if #[cfg(feature = "webgl_2")] {
        use super::{WebGlContext};

        pub fn vertex_attrib_divisor_direct(gl:&WebGlContext, loc: u32, divisor: u32) {
            gl.vertex_attrib_divisor(loc, divisor);
        }

        pub fn draw_arrays_instanced_direct(gl:&WebGlContext, mode: BeginMode, first: u32, count: u32, primcount: u32) {
            gl.draw_arrays_instanced( mode as u32, first as i32, count as i32, primcount as i32);
        }

        pub fn draw_elements_instanced_direct(gl:&WebGlContext, mode: BeginMode, count: u32, data_type: DataType, offset: u32, primcount: u32) {
                gl.draw_elements_instanced_with_i32( mode as u32, count as i32, data_type as u32, offset as i32, primcount as i32);
        }

        impl WebGlRenderer {
            pub fn vertex_attrib_divisor(&self, loc: u32, divisor: u32) -> Result<(), Error> {
                vertex_attrib_divisor_direct(&self.gl, loc, divisor);
                Ok(())
            }

            pub fn draw_arrays_instanced(&self, mode: BeginMode, first: u32, count: u32, primcount: u32) -> Result<(), Error> {
                draw_arrays_instanced_direct(&self.gl, mode, first, count, primcount);
                Ok(())
            }

            pub fn draw_elements_instanced(&mut self, mode: BeginMode, count: u32, data_type: DataType, offset: u32, primcount: u32) -> Result<(), Error> {
                draw_elements_instanced_direct(&self.gl, mode, count, data_type, offset, primcount);

                Ok(())
            }
        }
    } 
}

