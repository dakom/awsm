use crate::events;
use awsm_renderer::transform::Vector3;
use shipyard::prelude::*;

pub struct State {
    pub camera_settings: Option<CameraSettings>,
    pub window_size: events::WindowSize,
    pub camera_node: Option<Key>
}

pub enum CameraSettings {
    Orthographic(OrthographicCamera),
    Perspective(PerspectiveCamera)
}

pub struct OrthographicCamera {
    pub xmag: f64,
    pub ymag: f64,
    pub znear: f64,
    pub zfar: f64,
    pub translation: Vector3,
}

pub struct PerspectiveCamera {
    pub aspectRatio: f64,
    pub yfov: f64,
    pub znear: f64,
    pub zfar: f64,
    pub translation: Vector3,
}
