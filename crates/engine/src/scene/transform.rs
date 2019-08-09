use nalgebra::{Point3, Matrix4};

pub struct Transform {
    translation: Point3<f64>,
    rotation: Point3<f64>,
    scale: Point3<f64>,

    local_matrix: Matrix4<f64>,
    model_matrix: Matrix4<f64>,
    normal_matrix: Option<Matrix4<f64>>,
    model_view_matrix: Option<Matrix4<f64>>,
    model_view_projection_matrix: Option<Matrix4<f64>>,
}
