use crate::transform::*;
use crate::renderer::Renderer;
use shipyard::*;
use awsm_web::webgl::{ WebGl2Renderer, ClearBufferMask, BufferData, BufferTarget, BufferUsage, Id};

pub struct CameraView(pub Matrix4); 
pub struct CameraProjection(pub Matrix4); 

pub fn get_orthographic_projection(xmag:f64, ymag: f64, znear: f64, zfar: f64) -> Matrix4 {
    let mut projection = Matrix4::default();
    let mut values = projection.as_mut();

    values[0] = 1.0/xmag;
    values[5] = 1.0/ymag;
    values[10] = 2.0/(znear - zfar);
    values[14] = (zfar+znear) / (znear-zfar);
    values[15] = 1.0;

    projection
}

pub fn get_perspective_projection(aspect_ratio:f64, yfov: f64, znear: f64, zfar: Option<f64>) -> Matrix4 {
    let mut projection = Matrix4::default();
    let mut values = projection.as_mut();

    match zfar {
        None => {
            values[10] = -1.0;
            values[14] = (-2.0 * znear);
        },
        Some(zfar) => { 
            values[10] = (zfar+znear)/(znear-zfar);
            values[14] = (2.0 * zfar * znear)/(znear - zfar);
        }
    };

    values[0] = 1.0/(aspect_ratio * (0.5 * yfov).tan());
    values[5] = 1.0/((0.5 * yfov).tan());
    values[11] = -1.0;

    projection
}

pub fn get_view_from_local_mat(local_matrix:&Matrix4) -> Matrix4 {
    //TODO - should be inverse of this, I think?
    local_matrix.clone()
}

impl Renderer {
    /// if no node is provided then the first node will be used 
    pub fn update_camera_projection(&mut self, node: Option<Key>, projection:&[f64]) {
        let world = self.world.borrow_mut();
        world.run::<&mut CameraProjection, _>(|mut projs| {
            match node {
                Some(entity) => {
                    if let Some(proj) = (&mut projs).get(entity).iter_mut().next() {
                        proj.0.as_mut().copy_from_slice(projection);
                    } 
                },
                None => {
                    if let Some(proj) = projs.iter().next() {
                        proj.0.as_mut().copy_from_slice(projection);
                    }
                }
            }
        });
    }
    /// if no node is provided then the first node will be used 
    pub fn update_camera_view(&mut self, node: Option<Key>) {
        let world = self.world.borrow_mut();
        world.run::<(&mut CameraView, &LocalMatrix), _>(|(mut views, local_mats)| {

            match node {
                Some(entity) => {
                    if let Some((view, local_mat)) = (&mut views, &local_mats).get(entity).iter_mut().next() {
                        (*view).0 = get_view_from_local_mat(&local_mat.0);
                    } 
                },
                None => {
                    if let Some((view, local_mat)) = (&mut views, &local_mats).iter().next() {
                        (*view).0 = get_view_from_local_mat(&local_mat.0);
                    }
                }
            }
        });
    }

    /// if no node is provided then the first node will be used 
    pub(crate) fn update_camera_ubo(&mut self, node:Option<Key>) {
        let world = self.world.borrow_mut();
        let webgl = self.webgl.borrow_mut();

        world.run::<(&CameraView, &CameraProjection), _>(|(views, projs)| {
            if let Some((view, proj)) = (views, projs).iter().next() {
                let view = &view.0;
                let projection = &proj.0;
                
                let camera = vec![view.to_vec_f32(), projection.to_vec_f32()].concat();
                webgl.upload_buffer(
                    self.camera_buffer_id,
                    BufferData::new(
                        &camera,
                        BufferTarget::UniformBuffer,
                        BufferUsage::DynamicDraw,
                    ),
                ).unwrap();
            }
        });
    }
}