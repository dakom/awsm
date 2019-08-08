use nalgebra::{Matrix4, Point3};

pub struct Camera {
    pub gltf_index: Option<usize>,
    pub settings: Option<CameraSettings>,
    pub position: Option<Point3<f64>>,
    pub view: Matrix4<f64>, 
    pub projection: Matrix4<f64>, 
}

pub enum CameraSettings {
    Orthographic(OrthographicCamera),
    Perspective(PerspectiveCamera),
}

pub struct OrthographicCamera {
    pub x_mag: f64,
    pub y_mag: f64,
    pub z_near: f64,
    pub z_far: f64,
}

pub struct PerspectiveCamera {
    pub aspect_ratio: f64,
    pub y_fov: f64,
    pub z_near: f64,
    pub z_far: f64,
}
