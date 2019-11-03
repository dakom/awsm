pub trait SliceValues {
    fn values(self: &Self) -> &[f64] {
        let pointer = self as *const Self as *const f64;
        let slice: &[f64] = unsafe { std::slice::from_raw_parts(pointer, 4) };
        slice
    }
    fn to_vec_f32(self: &Self) -> Vec<f32> {
        self.values().iter().map(|n| *n as f32).collect()
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct Area {
    pub width: f64,
    pub height: f64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct Volume {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

impl Area {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}
impl SliceValues for Area {}

impl Volume {
    pub fn new(width: f64, height: f64, depth: f64) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}

impl SliceValues for Volume {}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }
}

impl SliceValues for Color {}
