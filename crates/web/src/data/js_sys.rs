use js_sys::{
    ArrayBuffer, Float32Array, Float64Array, Int16Array, Int32Array, Int8Array, Object,
    Uint16Array, Uint32Array, Uint8Array,
};
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

pub fn clone_to_vec_f32(src: &Float32Array) -> Vec<f32> {
    let mut dest: Vec<f32> = vec![0.0; src.length() as usize];
    src.copy_to(&mut dest);
    dest
}

pub fn clone_to_vec_f64(src: &Float64Array) -> Vec<f64> {
    let mut dest: Vec<f64> = vec![0.0; src.length() as usize];
    src.copy_to(&mut dest);
    dest
}

pub fn clone_to_vec_u8(src: &Uint8Array) -> Vec<u8> {
    let mut dest: Vec<u8> = vec![0; src.length() as usize];
    src.copy_to(&mut dest);
    dest
}
pub fn clone_to_vec_u16(src: &Uint16Array) -> Vec<u16> {
    let mut dest: Vec<u16> = vec![0; src.length() as usize];
    src.copy_to(&mut dest);
    dest
}

pub fn clone_to_vec_u32(src: &Uint32Array) -> Vec<u32> {
    let mut dest: Vec<u32> = vec![0; src.length() as usize];
    src.copy_to(&mut dest);
    dest
}

pub fn clone_to_vec_i8(src: &Int8Array) -> Vec<i8> {
    let mut dest: Vec<i8> = vec![0; src.length() as usize];
    src.copy_to(&mut dest);
    dest
}

pub fn clone_to_vec_i16(src: &Int16Array) -> Vec<i16> {
    let mut dest: Vec<i16> = vec![0; src.length() as usize];
    src.copy_to(&mut dest);
    dest
}

pub fn clone_to_vec_i32(src: &Int32Array) -> Vec<i32> {
    let mut dest: Vec<i32> = vec![0; src.length() as usize];
    src.copy_to(&mut dest);
    dest
}

//newtype wrapper
pub struct TypedData<T, U>(T, PhantomData<U>);
impl<T: AsRef<[U]>, U> TypedData<T, U> {
    pub fn new(values: T) -> Self {
        Self(values, PhantomData)
    }
}

//implementations for different data types as ArrayBuffer
impl<T: AsRef<[i8]>> From<TypedData<T, i8>> for ArrayBuffer {
    fn from(data: TypedData<T, i8>) -> Self {
        unsafe { js_sys::Int8Array::view(data.0.as_ref()).buffer() }
    }
}

impl<T: AsRef<[u8]>> From<TypedData<T, u8>> for ArrayBuffer {
    fn from(data: TypedData<T, u8>) -> Self {
        unsafe { js_sys::Uint8Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[i16]>> From<TypedData<T, i16>> for ArrayBuffer {
    fn from(data: TypedData<T, i16>) -> Self {
        unsafe { js_sys::Int16Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[u16]>> From<TypedData<T, u16>> for ArrayBuffer {
    fn from(data: TypedData<T, u16>) -> Self {
        unsafe { js_sys::Uint16Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[i32]>> From<TypedData<T, i32>> for ArrayBuffer {
    fn from(data: TypedData<T, i32>) -> Self {
        unsafe { js_sys::Int32Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[u32]>> From<TypedData<T, u32>> for ArrayBuffer {
    fn from(data: TypedData<T, u32>) -> Self {
        unsafe { js_sys::Uint32Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[f32]>> From<TypedData<T, f32>> for ArrayBuffer {
    fn from(data: TypedData<T, f32>) -> Self {
        unsafe { js_sys::Float32Array::view(data.0.as_ref()).buffer() }
    }
}
impl<T: AsRef<[f64]>> From<TypedData<T, f64>> for ArrayBuffer {
    fn from(data: TypedData<T, f64>) -> Self {
        unsafe { js_sys::Float64Array::view(data.0.as_ref()).buffer() }
    }
}

//implementations for different data types as JsValue
impl<T: AsRef<[i8]>> From<TypedData<T, i8>> for JsValue {
    fn from(data: TypedData<T, i8>) -> Self {
        unsafe { js_sys::Int8Array::view(data.0.as_ref()).into() }
    }
}

impl<T: AsRef<[u8]>> From<TypedData<T, u8>> for JsValue {
    fn from(data: TypedData<T, u8>) -> Self {
        unsafe { js_sys::Uint8Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[i16]>> From<TypedData<T, i16>> for JsValue {
    fn from(data: TypedData<T, i16>) -> Self {
        unsafe { js_sys::Int16Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[u16]>> From<TypedData<T, u16>> for JsValue {
    fn from(data: TypedData<T, u16>) -> Self {
        unsafe { js_sys::Uint16Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[i32]>> From<TypedData<T, i32>> for JsValue {
    fn from(data: TypedData<T, i32>) -> Self {
        unsafe { js_sys::Int32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[u32]>> From<TypedData<T, u32>> for JsValue {
    fn from(data: TypedData<T, u32>) -> Self {
        unsafe { js_sys::Uint32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[f32]>> From<TypedData<T, f32>> for JsValue {
    fn from(data: TypedData<T, f32>) -> Self {
        unsafe { js_sys::Float32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[f64]>> From<TypedData<T, f64>> for JsValue {
    fn from(data: TypedData<T, f64>) -> Self {
        unsafe { js_sys::Float64Array::view(data.0.as_ref()).into() }
    }
}

//implementations for different data types as Object
impl<T: AsRef<[i8]>> From<TypedData<T, i8>> for Object {
    fn from(data: TypedData<T, i8>) -> Self {
        unsafe { js_sys::Int8Array::view(data.0.as_ref()).into() }
    }
}

impl<T: AsRef<[u8]>> From<TypedData<T, u8>> for Object {
    fn from(data: TypedData<T, u8>) -> Self {
        unsafe { js_sys::Uint8Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[i16]>> From<TypedData<T, i16>> for Object {
    fn from(data: TypedData<T, i16>) -> Self {
        unsafe { js_sys::Int16Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[u16]>> From<TypedData<T, u16>> for Object {
    fn from(data: TypedData<T, u16>) -> Self {
        unsafe { js_sys::Uint16Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[i32]>> From<TypedData<T, i32>> for Object {
    fn from(data: TypedData<T, i32>) -> Self {
        unsafe { js_sys::Int32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[u32]>> From<TypedData<T, u32>> for Object {
    fn from(data: TypedData<T, u32>) -> Self {
        unsafe { js_sys::Uint32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[f32]>> From<TypedData<T, f32>> for Object {
    fn from(data: TypedData<T, f32>) -> Self {
        unsafe { js_sys::Float32Array::view(data.0.as_ref()).into() }
    }
}
impl<T: AsRef<[f64]>> From<TypedData<T, f64>> for Object {
    fn from(data: TypedData<T, f64>) -> Self {
        unsafe { js_sys::Float64Array::view(data.0.as_ref()).into() }
    }
}
