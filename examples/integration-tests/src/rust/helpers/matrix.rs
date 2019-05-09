pub fn write_scale_matrix(width:f64, height:f64, depth:f64, out:&mut [f32]) {
  out[0] = width as f32;
  out[1] = 0.0;
  out[2] = 0.0;
  out[3] = 0.0;
  out[4] = 0.0;
  out[5] = height as f32;
  out[6] = 0.0;
  out[7] = 0.0;
  out[8] = 0.0;
  out[9] = 0.0;
  out[10] = depth as f32;
  out[11] = 0.0;
  out[12] = 0.0;
  out[13] = 0.0;
  out[14] = 0.0;
  out[15] = 1.0;
}


pub fn write_position_matrix(x:f64, y:f64, z:f64, out:&mut [f32]) {
  out[0] = 1.0;
  out[1] = 0.0;
  out[2] = 0.0;
  out[3] = 0.0;
  out[4] = 0.0;
  out[5] = 1.0;
  out[6] = 0.0;
  out[7] = 0.0;
  out[8] = 0.0;
  out[9] = 0.0;
  out[10] = 1.0;
  out[11] = 0.0;
  out[12] = x as f32;
  out[13] = y as f32;
  out[14] = z as f32;
  out[15] = 1.0;
}

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

pub fn write_multiply_matrix(a:&[f32;16], b:&[f32;16], out:&mut [f32]) {

  let a00 = a[0]; 
  let a01 = a[1]; 
  let a02 = a[2]; 
  let a03 = a[3];
  let a10 = a[4]; 
  let a11 = a[5]; 
  let a12 = a[6]; 
  let a13 = a[7];
  let a20 = a[8]; 
  let a21 = a[9]; 
  let a22 = a[10]; 
  let a23 = a[11];
  let a30 = a[12]; 
  let a31 = a[13]; 
  let a32 = a[14]; 
  let a33 = a[15];
  // Cache only the current line of the second matrix
  let mut b0  = b[0]; 
  let mut b1 = b[1]; 
  let mut b2 = b[2]; 
  let mut b3 = b[3];
  out[0] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
  out[1] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
  out[2] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
  out[3] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
  b0 = b[4]; b1 = b[5]; b2 = b[6]; b3 = b[7];
  out[4] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
  out[5] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
  out[6] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
  out[7] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
  b0 = b[8]; b1 = b[9]; b2 = b[10]; b3 = b[11];
  out[8] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
  out[9] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
  out[10] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
  out[11] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
  b0 = b[12]; b1 = b[13]; b2 = b[14]; b3 = b[15];
  out[12] = b0*a00 + b1*a10 + b2*a20 + b3*a30;
  out[13] = b0*a01 + b1*a11 + b2*a21 + b3*a31;
  out[14] = b0*a02 + b1*a12 + b2*a22 + b3*a32;
  out[15] = b0*a03 + b1*a13 + b2*a23 + b3*a33;
}
