use super::{WebGlCommon, WebGlRenderer};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

pub trait PartialWebGlMisc {
    fn awsm_depth_mask(&self, flag: bool);
    fn awsm_clear_color(&self, r: f32, g: f32, b: f32, a: f32);
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlMisc for $type {
            fn awsm_depth_mask(&self, flag:bool) {
                self.depth_mask(flag);
            }
            fn awsm_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
                self.clear_color(r, g, b, a);
            }
            $($defs)*
        })+
    };
}

pub struct MiscSettings {
    pub depth_mask: bool,
    pub clear_color: (f32, f32, f32, f32),
}

impl Default for MiscSettings {
    fn default() -> Self {
        Self {
            depth_mask: true,
            clear_color: (0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl_context! {
    WebGlRenderingContext{}
    WebGl2RenderingContext{}
}

impl<T: WebGlCommon> WebGlRenderer<T> {
    pub fn set_depth_mask(&mut self, flag: bool) {
        if self.misc_settings.depth_mask != flag {
            self.gl.awsm_depth_mask(flag);
            self.misc_settings.depth_mask = flag;
        }
    }

    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let curr = self.misc_settings.clear_color;

        if curr.0 != r || curr.1 != g || curr.2 != b || curr.3 != a {
            self.gl.awsm_clear_color(r, g, b, a);
            self.misc_settings.clear_color = (r, g, b, a);
        }
    }
}
