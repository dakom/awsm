use super::slice::{SliceValues};

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    pub x:f64,
    pub y:f64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    pub r:f64,
    pub g:f64,
    pub b:f64,
    pub a:f64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
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
