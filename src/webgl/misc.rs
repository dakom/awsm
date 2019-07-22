use super::{WebGlCommon, WebGlRenderer};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

pub trait PartialWebGlMisc {
    fn awsm_depth_mask(&self, flag: bool);
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlMisc for $type {
            fn awsm_depth_mask(&self, flag:bool) {
                self.depth_mask(flag);
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
    pub fn set_depth_mask(&mut self, flag: bool) {
        if self.depth_mask != flag {
            self.gl.awsm_depth_mask(flag);
            self.depth_mask = flag;
        }
    }
}
