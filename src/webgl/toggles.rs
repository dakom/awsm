use super::{GlToggle, WebGlCommon, WebGlRenderer};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

pub trait PartialWebGlToggle {
    fn awsm_enable(&self, toggle: GlToggle);
    fn awsm_disable(&self, toggle: GlToggle);
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlToggle for $type {
            fn awsm_enable(&self, toggle:GlToggle) {
                self.enable(toggle as u32);
            }
            fn awsm_disable(&self, toggle:GlToggle) {
                self.disable(toggle as u32);
            }
            $($defs)*
        })+
    };
}

impl_context! {
    WebGlRenderingContext{}
    WebGl2RenderingContext{}
}
#[derive(Default)]
pub(super) struct ToggleFlags {
    blend: bool,
    cull_face: bool,
    depth_test: bool,
    dither: bool,
    polygon_offset_fill: bool,
    sample_alpha_to_coverage: bool,
    sample_coverage: bool,
    scissor_test: bool,
    stencil_test: bool,
    rasterizer_discard: bool,
}

impl<T: WebGlCommon> WebGlRenderer<T> {
    pub fn toggle(&mut self, toggle: GlToggle, flag: bool) {
        let curr = match toggle {
            GlToggle::Blend => &mut self.toggle_flags.blend,
            GlToggle::CullFace => &mut self.toggle_flags.cull_face,
            GlToggle::DepthTest => &mut self.toggle_flags.depth_test,
            GlToggle::Dither => &mut self.toggle_flags.dither,
            GlToggle::PolygonOffsetFill => &mut self.toggle_flags.polygon_offset_fill,
            GlToggle::SampleAlphaToCoverage => &mut self.toggle_flags.sample_alpha_to_coverage,
            GlToggle::SampleCoverage => &mut self.toggle_flags.sample_coverage,
            GlToggle::ScissorTest => &mut self.toggle_flags.scissor_test,
            GlToggle::StencilTest => &mut self.toggle_flags.stencil_test,
            GlToggle::RasterizerDiscard => &mut self.toggle_flags.rasterizer_discard,
        };

        if *curr != flag {
            *curr = flag;
            match flag {
                true => self.gl.awsm_enable(toggle),
                false => self.gl.awsm_disable(toggle),
            };
        }
    }
}
