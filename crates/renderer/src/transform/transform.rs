use nalgebra::{Vector3, Matrix4, UnitQuaternion};

pub struct Translation(Vector3<f64>);
pub struct Rotation(UnitQuaternion<f64>);
pub struct Scale(Vector3<f64>);
pub struct LocalMatrix(Matrix4<f32>);
pub struct WorldMatrix(Matrix4<f32>);

pub fn trs_to_mat(translation:&Vector3<f64>, rotation:&UnitQuaternion<f64>, scale:&Vector3<f64>) -> Matrix4<f32> {

    let t = Matrix4::new_translation(&Vector3::new(translation.x as f32, translation.y as f32, translation.z as f32));
    //TODO - fix this..
    let r = UnitQuaternion::new(rotation[0] as f32, rotation[1] as f32, rotation[2] as f32, rotation[3] as f32).to_rotation_matrix().to_homogeneous();
                //let s = Matrix4::from_nonuniform_scale(s[0], s[1], s[2]);
                //(t * r * s).as_array()

    t
}