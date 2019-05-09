
#[repr(C)]
pub struct Point {
    pub x:f64,
    pub y:f64,
}

#[repr(C)]
pub struct Color {
    pub r:f64,
    pub g:f64,
    pub b:f64,
    pub a:f64,
}

#[repr(C)]
pub struct Area {
    pub width:f64,
    pub height:f64,
}

impl SliceValues for Point {}

impl SliceValues for Color {}

impl SliceValues for Area {}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color { r, g, b, a, }
    }

}

pub trait SliceValues {
    fn values(self:&Self) -> &[f64] {
        let pointer = self as *const Self as *const f64;
        let slice: &[f64] = unsafe { std::slice::from_raw_parts(pointer, 4) };
        slice
    }

    fn write_to_v32_4(self:&Self, target:&mut [f32]) {
        let values = self.values();
        target[0] = values[0] as f32;
        target[1] = values[1] as f32;
        target[2] = values[2] as f32;
        target[3] = values[3] as f32;
    }
    fn write_to_v32_16(self:&Self, target:&mut [f32]) {
        let values = self.values();
        target[0] = values[0] as f32;
        target[1] = values[1] as f32;
        target[2] = values[2] as f32;
        target[3] = values[3] as f32;
        
        target[4] = values[4] as f32;
        target[5] = values[5] as f32;
        target[6] = values[6] as f32;
        target[7] = values[7] as f32;

        target[8] = values[8] as f32;
        target[9] = values[9] as f32;
        target[10] = values[10] as f32;
        target[11] = values[11] as f32;

        target[12] = values[12] as f32;
        target[13] = values[13] as f32;
        target[14] = values[14] as f32;
        target[15] = values[15] as f32;
    }
}
