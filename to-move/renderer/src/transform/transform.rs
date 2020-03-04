/*
    The math was taken and adapted from various places on the internet
    Specifically, from gl-matrix and the gltf-rs crate (which in turn took from cg_math)
*/
use crate::errors::{Error, NativeError};
pub struct Translation(pub Vector3);
pub struct Rotation(pub Quaternion);
pub struct Scale(pub Vector3);
pub struct LocalTransform(pub Matrix4);
pub struct WorldTransform(pub Matrix4);

#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}
impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{x, y, z}
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}
impl TransformValues for Vector3 {
    fn len(&self) -> usize { 3 }
}

#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct Quaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
impl Quaternion {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self{x, y, z, w}
    }


}
impl Default for Quaternion {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}
impl TransformValues for Quaternion {
    fn len(&self) -> usize { 4 }
}

#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct Matrix4 (
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
);

impl Default for Matrix4 {
    fn default() -> Self {
        Self(
            1.0,0.0,0.0,0.0,
            0.0,1.0,0.0,0.0,
            0.0,0.0,1.0,0.0,
            0.0,0.0,0.0,1.0,
        )
    }
}

impl Matrix4 {

    pub fn from_translation(v: &Vector3) -> Self {
        Self(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            v.x, v.y, v.z, 1.0,
        )
    }
    pub fn from_rotation(r: &Quaternion) -> Self {
            let x2 = r.x + r.x;
            let y2 = r.y + r.y;
            let z2 = r.z + r.z;

            let xx2 = x2 * r.x;
            let xy2 = x2 * r.y;
            let xz2 = x2 * r.z;

            let yy2 = y2 * r.y;
            let yz2 = y2 * r.z;
            let zz2 = z2 * r.z;

            let sy2 = y2 * r.w;
            let sz2 = z2 * r.w;
            let sx2 = x2 * r.w;

            Self(
                1.0 - yy2 - zz2, xy2 + sz2, xz2 - sy2, 0.0,
                xy2 - sz2, 1.0 - xx2 - zz2, yz2 + sx2, 0.0,
                xz2 + sy2, yz2 - sx2, 1.0 - xx2 - yy2, 0.0,
                0.0, 0.0, 0.0, 1.0,
            )
    }

    pub fn set_from_scale(&mut self, scale:&Vector3) {
        self.reset();
        self.0 = scale.x;
        self.5 = scale.y;
        self.10 = scale.z;
    }
    pub fn from_scale(scale:&Vector3) -> Self {
        Self(
            scale.x, 0.0, 0.0, 0.0,
            0.0,   scale.y, 0.0, 0.0,
            0.0, 0.0,   scale.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
    pub fn from_trs_mut(&mut self, translation:&Vector3, rotation:&Quaternion, scale:&Vector3) {
        self.set_from_scale(scale);
        self.mul_mut(&Self::from_rotation(rotation));
        self.mul_mut(&Self::from_translation(translation));
    }

    pub fn from_trs(translation:&Vector3, rotation:&Quaternion, scale:&Vector3) -> Self {
        let mut _self = Self::from_scale(scale);
        _self.mul_mut(&Self::from_rotation(rotation));
        _self.mul_mut(&Self::from_translation(translation));
        _self
    }

    pub fn mul_mut(&mut self, rhs: &Matrix4) {
        let a:&[f64] = self.as_ref();
        let b:&[f64] = rhs.as_ref();
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
        let mut b0  = b[0];
        let mut b1 = b[1];
        let mut b2 = b[2];
        let mut b3 = b[3];

        self.0 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        self.1 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        self.2 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        self.3 = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[4]; b1 = b[5]; b2 = b[6]; b3 = b[7];
        self.4 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        self.5 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        self.6 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        self.7 = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[8]; b1 = b[9]; b2 = b[10]; b3 = b[11];
        self.8 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        self.9 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        self.10 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        self.11 = b0*a03 + b1*a13 + b2*a23 + b3*a33;
        b0 = b[12]; b1 = b[13]; b2 = b[14]; b3 = b[15];
        self.12 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
        self.13 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
        self.14 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
        self.15 = b0*a03 + b1*a13 + b2*a23 + b3*a33;
    }

    pub fn invert_clone(orig:&Self) -> Result<Self, Error> {
        let mut clone = orig.clone();
        clone.invert_mut()?;
        Ok(clone)
    }
    /// returns true if it was able to invert, false otherwise
    pub fn invert_mut(&mut self) -> Result<(), Error> {
        let a:&[f64] = self.as_ref();
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
        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;
        // Calculate the determinant
        let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
        if det == 0.0 {
            Err(NativeError::InvertMatrix.into())
        } else {
            det = 1.0 / det;
            self.0 = (a11 * b11 - a12 * b10 + a13 * b09) * det;
            self.1 = (a02 * b10 - a01 * b11 - a03 * b09) * det;
            self.2 = (a31 * b05 - a32 * b04 + a33 * b03) * det;
            self.3 = (a22 * b04 - a21 * b05 - a23 * b03) * det;
            self.4 = (a12 * b08 - a10 * b11 - a13 * b07) * det;
            self.5 = (a00 * b11 - a02 * b08 + a03 * b07) * det;
            self.6 = (a32 * b02 - a30 * b05 - a33 * b01) * det;
            self.7 = (a20 * b05 - a22 * b02 + a23 * b01) * det;
            self.8 = (a10 * b10 - a11 * b08 + a13 * b06) * det;
            self.9 = (a01 * b08 - a00 * b10 - a03 * b06) * det;
            self.10 = (a30 * b04 - a31 * b02 + a33 * b00) * det;
            self.11 = (a21 * b02 - a20 * b04 - a23 * b00) * det;
            self.12 = (a11 * b07 - a10 * b09 - a12 * b06) * det;
            self.13 = (a00 * b09 - a01 * b07 + a02 * b06) * det;
            self.14 = (a31 * b01 - a30 * b03 - a32 * b00) * det;
            self.15 = (a20 * b03 - a21 * b01 + a22 * b00) * det;
            Ok(())
        }
    }
}
impl TransformValues for Matrix4 {
    fn len(&self) -> usize { 16 }
}

impl std::ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut out = self.clone();
        out.mul_mut(&rhs);
        out
    }
}

pub trait TransformValues: AsRef<[f64]> + AsMut<[f64]> + Default {
    fn len(self: &Self) -> usize;

    //TODO: cache! maybe Cow?
    fn to_vec_f32(self: &Self) -> Vec<f32> {
        self.as_ref().iter().map(|n| *n as f32).collect()
    }

    fn copy_from_slice(&mut self, values:&[f64]) {
        let curr:&mut [f64] = self.as_mut(); 
        curr.copy_from_slice(values);
    }

    fn reset(&mut self) {
        //TODO: might be possible to keep this as like a static somehow?
        let _default = Self::default();
        self.copy_from_slice(_default.as_ref());
    }
    fn new_from_slice(values:&[f64]) -> Self {
        let mut _self = Self::default();
        _self.copy_from_slice(values);
        _self
    }

    fn copy_from(&mut self, other:&Self) {
        self.copy_from_slice(other.as_ref());
    }
}
macro_rules! impl_asref {
    ( $( $x:ty ),* ) => {
        $(

            impl AsRef<[f64]> for $x {
                //this is fast - no copy
                fn as_ref(&self) -> &[f64] {
                    let pointer = self as *const Self as *const f64;
                    let slice: &[f64] = unsafe { std::slice::from_raw_parts(pointer, self.len()) };
                    slice
                }
            }
            impl AsMut<[f64]> for $x {
                //this is fast - no copy
                fn as_mut(&mut self) -> &mut [f64] {
                    let pointer = self as *const Self as *mut f64;
                    let slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(pointer, self.len()) };
                    slice
                }
            }
        )*
    };
}

impl_asref!{Vector3, Quaternion, Matrix4}
