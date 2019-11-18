use nalgebra::{Matrix4, Point2, Vector3};

pub struct Camera {
    view:Matrix4<f32>,
    projection:Matrix4<f32>
}