use super::{BeginMode, ClearBufferMask, DataType, WebGlCommon, WebGlRenderer};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

pub trait PartialWebGlDrawing {
    fn awsm_clear(&self, bits: &[ClearBufferMask]);
    fn awsm_draw_arrays(&self, mode: BeginMode, first: u32, count: u32);
    fn awsm_draw_elements(&self, mode: BeginMode, count: u32, data_type: DataType, offset: u32);
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlDrawing for $type {

            fn awsm_clear(&self, bits: &[ClearBufferMask]) {
                let mut combined = 0u32;
                for bit in bits {
                    combined = combined | *bit as u32;
                }
                self.clear(combined);
            }

            fn awsm_draw_arrays(&self, mode: BeginMode, first: u32, count: u32) {
                self.draw_arrays(mode as u32, first as i32, count as i32);
            }

            fn awsm_draw_elements(&self, mode: BeginMode, count: u32, data_type:DataType, offset:u32) {
                self.draw_elements_with_i32(mode as u32, count as i32, data_type as u32, offset as i32);
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
    pub fn clear(&self, bits: &[ClearBufferMask]) {
        self.gl.awsm_clear(&bits);
    }

    pub fn draw_arrays(&self, mode: BeginMode, first: u32, count: u32) {
        self.gl.awsm_draw_arrays(mode, first, count);
    }

    pub fn draw_elements(&self, mode: BeginMode, count: u32, data_type: DataType, offset: u32) {
        self.gl.awsm_draw_elements(mode, count, data_type, offset);
    }
}
