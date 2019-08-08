use super::{WebGlCommon, WebGlRenderer};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

pub trait PartialWebGlViewport {
    fn awsm_viewport(&self, x: i32, y: i32, width: i32, height: i32);
    fn awsm_drawing_buffer_height(&self) -> i32;
    fn awsm_drawing_buffer_width(&self) -> i32;
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlViewport for $type {
            fn awsm_viewport(&self, x: i32, y: i32, width: i32, height: i32) {
                self.viewport(x, y, width, height)
            }
            fn awsm_drawing_buffer_height(&self) -> i32 {
                self.drawing_buffer_height()
            }
            fn awsm_drawing_buffer_width(&self) -> i32 {
                self.drawing_buffer_width()
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
    pub fn resize(&mut self, width: u32, height: u32) {
        if self.last_width != width || self.last_height != height {
            let gl = &mut self.gl;
            let canvas = &mut self.canvas;
            canvas.set_width(width);
            canvas.set_height(height);
            gl.awsm_viewport(
                0,
                0,
                gl.awsm_drawing_buffer_width(),
                gl.awsm_drawing_buffer_height(),
            );
            self.last_width = width;
            self.last_height = height;
        }
    }

    pub fn current_size(&self) -> (u32, u32) {
        (self.last_width, self.last_height)
    }
}
