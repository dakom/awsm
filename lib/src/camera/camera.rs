pub fn write_ortho(left:f64, right:f64, bottom:f64, top:f64, near:f64, far:f64, out:&mut [f32]) {
  let lr = 1.0 / (left - right);
  let bt = 1.0 / (bottom - top);
  let nf = 1.0 / (near - far);
  out[0] = -2.0 * lr as f32;
  out[1] = 0.0;
  out[2] = 0.0;
  out[3] = 0.0;
  out[4] = 0.0;
  out[5] = -2.0 * bt as f32;
  out[6] = 0.0;
  out[7] = 0.0;
  out[8] = 0.0;
  out[9] = 0.0;
  out[10] = 2.0 * nf as f32;
  out[11] = 0.0;
  out[12] = ((left + right) * lr) as f32;
  out[13] = ((top + bottom) * bt) as f32;
  out[14] = ((far + near) * nf) as f32;
  out[15] = 1.0;
}
