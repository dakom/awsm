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

pub fn copy_slices_to_slice<T>(srcs:&[Vec<T>], dest:&mut [T]) 
where T: Copy
{
    let mut offset = 0;
    for src in srcs.iter() {
        let len = src.len();
        let max = offset + len;
        dest[offset..max].copy_from_slice(&src);
        offset = max;
    }
}
