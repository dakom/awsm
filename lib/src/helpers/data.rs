use super::slice::{SliceValues};

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    pub x:f64,
    pub y:f64,
    pub z:f64
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct Point2d {
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct Volume {
    pub width:f64,
    pub height:f64,
    pub depth:f64,
}

impl SliceValues for Point {}
impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x,y,z}
    }
}

impl SliceValues for Point2d {}
impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x,y}
    }
}

impl SliceValues for Area {}
impl Area {
    pub fn new(width: f64, height: f64) -> Self {
        Self{width, height}
    }
}

impl SliceValues for Volume {}
impl Volume{
    pub fn new(width: f64, height: f64, depth:f64) -> Self {
        Self{width, height, depth}
    }
}

impl SliceValues for Color {}
impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a, }
    }
}

