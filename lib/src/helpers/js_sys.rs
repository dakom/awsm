use js_sys::{Float32Array, Float64Array};

pub fn clone_to_vec_f32(src:&Float32Array) -> Vec<f32> {
    let mut dest:Vec<f32> = vec![0.0; src.length() as usize];
    src.copy_to(&mut dest);
    dest
}

pub fn clone_to_vec_f64(src:&Float64Array) -> Vec<f64> {
    let mut dest:Vec<f64> = vec![0.0; src.length() as usize];
    src.copy_to(&mut dest);
    dest
}
