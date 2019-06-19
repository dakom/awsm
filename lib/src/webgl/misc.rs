use super::{WebGlRenderer};

impl WebGlRenderer {
    pub fn set_depth_mask(&mut self, flag:bool) {
        if self.depth_mask != flag {
            self.gl.depth_mask(flag);
            self.depth_mask = flag;
        }
    }
}
