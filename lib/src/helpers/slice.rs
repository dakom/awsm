pub trait SliceValues {
    fn values(self:&Self) -> &[f64] {
        let pointer = self as *const Self as *const f64;
        let slice: &[f64] = unsafe { std::slice::from_raw_parts(pointer, 4) };
        slice
    }

    fn values_as_f32(self:&Self) -> Vec<f32> {
        self.values().iter().map(|n| *n as f32).collect()
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
