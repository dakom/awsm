use crate::helpers::{identity};

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

pub fn write_persp(fovy:f64, aspect:f64, near:f64, far:f64, out:&mut [f32]) {
    let f = 1.0 / (fovy / 2.0).tan();
    let mut nf:f64;
    out[0] = (f / aspect) as f32;
    out[1] = 0.0;
    out[2] = 0.0;
    out[3] = 0.0;
    out[4] = 0.0;
    out[5] = f as f32;
    out[6] = 0.0;
    out[7] = 0.0;
    out[8] = 0.0;
    out[9] = 0.0;
    out[11] = -1.0;
    out[12] = 0.0;
    out[13] = 0.0;
    out[15] = 0.0;
    if (!far.is_infinite()) {
        nf = 1.0 / (near - far);
        out[10] = ((far + near) * nf) as f32;
        out[14] = ((2.0 * far * near) * nf) as f32;
    } else {
        out[10] = -1.0;
        out[14] = (-2.0 * near) as f32;
    }
}
//probably a better way to do this so just keeping private for now
//the native rust type only allows a.hypot(b)
fn hypot3(a:f64, b:f64, c:f64) -> f64 {
    (a.powi(2) + b.powi(2) + c.powi(2)).sqrt()
}

pub fn look_at(eye:&[f64], center:&[f64], up:&[f64], out:&mut[f32]) {
  let mut x0:f64;
  let mut x1:f64;
  let mut x2:f64; 
  let mut y0:f64; 
  let mut y1:f64; 
  let mut y2:f64;
  let mut z0:f64;
  let mut z1:f64; 
  let mut z2:f64; 
  let mut len:f64;

  let eyex = eye[0];
  let eyey = eye[1];
  let eyez = eye[2];
  let upx = up[0];
  let upy = up[1];
  let upz = up[2];
  let centerx = center[0];
  let centery = center[1];
  let centerz = center[2];
  if f64::abs(eyex - centerx) < std::f64::EPSILON 
        && f64::abs(eyey - centery) < std::f64::EPSILON 
        && f64::abs(eyez - centerz) < std::f64::EPSILON {
    return identity(out);
  }
  z0 = eyex - centerx;
  z1 = eyey - centery;
  z2 = eyez - centerz;
  len = 1.0 / hypot3(z0, z1, z2);
  z0 *= len;
  z1 *= len;
  z2 *= len;
  x0 = upy * z2 - upz * z1;
  x1 = upz * z0 - upx * z2;
  x2 = upx * z1 - upy * z0;
  len = hypot3(x0, x1, x2);
  if (len == 0.0) {
    x0 = 0.0;
    x1 = 0.0;
    x2 = 0.0;
  } else {
    len = 1.0 / len;
    x0 *= len;
    x1 *= len;
    x2 *= len;
  }
  y0 = z1 * x2 - z2 * x1;
  y1 = z2 * x0 - z0 * x2;
  y2 = z0 * x1 - z1 * x0;
  len = hypot3(y0, y1, y2);
  if (len == 0.0) {
    y0 = 0.0;
    y1 = 0.0;
    y2 = 0.0;
  } else {
    len = 1.0 / len;
    y0 *= len;
    y1 *= len;
    y2 *= len;
  }
  out[0] = x0 as f32;
  out[1] = y0 as f32;
  out[2] = z0 as f32;
  out[3] = 0.0f32;
  out[4] = x1 as f32;
  out[5] = y1 as f32;
  out[6] = z1 as f32;
  out[7] = 0.0;
  out[8] = x2 as f32;
  out[9] = y2 as f32;
  out[10] = z2 as f32;
  out[11] = 0.0;
  out[12] = -(x0 * eyex + x1 * eyey + x2 * eyez) as f32;
  out[13] = -(y0 * eyex + y1 * eyey + y2 * eyez) as f32;
  out[14] = -(z0 * eyex + z1 * eyey + z2 * eyez) as f32;
  out[15] = 1.0;
}
