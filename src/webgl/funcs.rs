use super::{BlendEquation, BlendFactor, CmpFunction, WebGlCommon, WebGlRenderer};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

pub trait PartialWebGlFuncs {
    fn awsm_depth_func(&self, func: CmpFunction);
    fn awsm_blend_color(&self, r: f32, g: f32, b: f32, a: f32);
    fn awsm_blend_func(&self, sfactor: BlendFactor, dfactor: BlendFactor);
    fn awsm_blend_func_separate(
        &self,
        src_rgb: BlendFactor,
        dest_rgb: BlendFactor,
        src_alpha: BlendFactor,
        dest_alpha: BlendFactor,
    );
    fn awsm_blend_equation(&self, mode: BlendEquation);
    fn awsm_blend_equation_separate(&self, rgb_mode: BlendEquation, alpha_mode: BlendEquation);
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlFuncs for $type {
            fn awsm_depth_func(&self, func:CmpFunction) {
                self.depth_func(func as u32);
            }
            fn awsm_blend_color(&self, r:f32, g:f32, b:f32, a:f32) {
                self.blend_color(r, g, b, a);
            }
            fn awsm_blend_func(&self, sfactor: BlendFactor, dfactor: BlendFactor) {
                self.blend_func(sfactor as u32, dfactor as u32);
            }
            fn awsm_blend_func_separate(&self, src_rgb: BlendFactor, dest_rgb: BlendFactor, src_alpha:BlendFactor, dest_alpha: BlendFactor) {
                self.blend_func_separate(src_rgb as u32, dest_rgb as u32, src_alpha as u32, dest_alpha as u32);
            }
            fn awsm_blend_equation(&self, mode:BlendEquation) {
                self.blend_equation(mode as u32);
            }
            fn awsm_blend_equation_separate(&self, rgb_mode:BlendEquation, alpha_mode:BlendEquation) {
                self.blend_equation_separate(rgb_mode as u32, alpha_mode as u32);
            }
            $($defs)*
        })+
    };
}

impl_context! {
    WebGlRenderingContext{}
    WebGl2RenderingContext{}
}

pub struct FuncSettings {
    pub depth_func: CmpFunction,
    pub blend_color: (f32, f32, f32, f32),
    pub blend_func: (BlendFactor, BlendFactor),
    pub blend_func_separate: (BlendFactor, BlendFactor, BlendFactor, BlendFactor),
    pub blend_equation: BlendEquation,
    pub blend_equation_separate: (BlendEquation, BlendEquation),
}

impl Default for FuncSettings {
    fn default() -> Self {
        Self {
            depth_func: CmpFunction::Less,
            blend_color: (0.0, 0.0, 0.0, 0.0),
            blend_func: (BlendFactor::One, BlendFactor::Zero),
            blend_func_separate: (
                BlendFactor::One,
                BlendFactor::Zero,
                BlendFactor::One,
                BlendFactor::Zero,
            ),
            blend_equation: BlendEquation::Add,
            blend_equation_separate: (BlendEquation::Add, BlendEquation::Add),
        }
    }
}

impl<T: WebGlCommon> WebGlRenderer<T> {
    pub fn set_depth_func(&mut self, func: CmpFunction) {
        if self.func_settings.depth_func != func {
            self.gl.awsm_depth_func(func);
            self.func_settings.depth_func = func;
        }
    }
    pub fn set_blend_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let curr = self.func_settings.blend_color;

        if curr.0 != r || curr.1 != g || curr.2 != b || curr.3 != a {
            self.gl.awsm_blend_color(r, g, b, a);
            self.func_settings.blend_color = (r, g, b, a);
        }
    }

    pub fn set_blend_func(&mut self, sfactor: BlendFactor, dfactor: BlendFactor) {
        let curr = self.func_settings.blend_func;

        if curr.0 != sfactor || curr.1 != dfactor {
            self.gl.awsm_blend_func(sfactor, dfactor);
            self.func_settings.blend_func = (sfactor, dfactor);
        }
    }
    pub fn set_blend_func_separate(
        &mut self,
        src_rgb: BlendFactor,
        dest_rgb: BlendFactor,
        src_alpha: BlendFactor,
        dest_alpha: BlendFactor,
    ) {
        let curr = self.func_settings.blend_func_separate;

        if curr.0 != src_rgb || curr.1 != dest_rgb || curr.2 != src_alpha || curr.3 != dest_alpha {
            self.gl
                .awsm_blend_func_separate(src_rgb, dest_rgb, src_alpha, dest_alpha);
            self.func_settings.blend_func_separate = (src_rgb, dest_rgb, src_alpha, dest_alpha);
        }
    }
    pub fn set_blend_equation(&mut self, mode: BlendEquation) {
        if self.func_settings.blend_equation != mode {
            self.gl.awsm_blend_equation(mode);
            self.func_settings.blend_equation = mode;
        }
    }
    pub fn set_blend_equation_separate(
        &mut self,
        rgb_mode: BlendEquation,
        alpha_mode: BlendEquation,
    ) {
        let curr = self.func_settings.blend_equation_separate;

        if curr.0 != rgb_mode || curr.1 != alpha_mode {
            self.gl.awsm_blend_equation_separate(rgb_mode, alpha_mode);
            self.func_settings.blend_equation_separate = (rgb_mode, alpha_mode);
        }
    }
}
