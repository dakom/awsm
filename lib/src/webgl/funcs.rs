use super::{WebGlRenderer, CmpFunction, BlendFactor, BlendEquation};


pub struct FuncSettings {
    pub depth_func: CmpFunction,
    pub blend_color: (f32, f32, f32, f32),
    pub blend_func: (BlendFactor, BlendFactor),
    pub blend_func_separate: (BlendFactor, BlendFactor, BlendFactor, BlendFactor),
    pub blend_equation: BlendEquation, 
    pub blend_equation_separate: (BlendEquation, BlendEquation) 
}

impl Default for FuncSettings {
    fn default() -> Self { 
        Self {
            depth_func: CmpFunction::Less,
            blend_color: (0.0, 0.0, 0.0, 0.0),
            blend_func: (BlendFactor::One, BlendFactor::Zero),
            blend_func_separate: (BlendFactor::One, BlendFactor::Zero, BlendFactor::One, BlendFactor::Zero),
            blend_equation: BlendEquation::Add,
            blend_equation_separate: (BlendEquation::Add, BlendEquation::Add) 
        }
    }
}

impl WebGlRenderer {
    //FUNCS
    pub fn set_depth_func(&mut self, func:CmpFunction) {
        if self.func_settings.depth_func != func {
            self.gl.depth_func(func as u32);
            self.func_settings.depth_func = func;
        }
    }
    pub fn set_blend_color(&mut self, r:f32, g:f32, b:f32, a:f32) {

        let curr = self.func_settings.blend_color;

        if curr.0 != r || curr.1 != g || curr.2 != b || curr.3 != a { 
            self.gl.blend_color(r, g, b, a);
            self.func_settings.blend_color= (r, g, b, a);
        }
    }

    pub fn set_blend_func(&mut self, sfactor: BlendFactor, dfactor: BlendFactor) {

        let curr = self.func_settings.blend_func;

        if curr.0 != sfactor || curr.1 != dfactor { 
            self.gl.blend_func(sfactor as u32, dfactor as u32);
            self.func_settings.blend_func = (sfactor, dfactor);
        }
    }
    pub fn set_blend_func_separate(&mut self, src_rgb: BlendFactor, dest_rgb: BlendFactor, src_alpha:BlendFactor, dest_alpha: BlendFactor) {

        let curr = self.func_settings.blend_func_separate;

        if curr.0 != src_rgb || curr.1 != dest_rgb || curr.2 != src_alpha || curr.3 != dest_alpha { 
            self.gl.blend_func_separate(src_rgb as u32, dest_rgb as u32, src_alpha as u32, dest_alpha as u32);
            self.func_settings.blend_func_separate = (src_rgb, dest_rgb, src_alpha, dest_alpha);
        }
    }
    pub fn set_blend_equation(&mut self, mode:BlendEquation) {
        if self.func_settings.blend_equation != mode {
            self.gl.blend_equation(mode as u32);
            self.func_settings.blend_equation = mode ;
        }
    }
    pub fn set_blend_equation_separate(&mut self, rgb_mode:BlendEquation, alpha_mode:BlendEquation) {

        let curr = self.func_settings.blend_equation_separate;

        if curr.0 != rgb_mode || curr.1 != alpha_mode { 
            self.gl.blend_equation_separate(rgb_mode as u32, alpha_mode as u32);
            self.func_settings.blend_equation_separate = (rgb_mode, alpha_mode);
        }
    }
}
