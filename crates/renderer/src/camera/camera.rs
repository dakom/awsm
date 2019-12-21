use crate::errors::Error;
use crate::transform::*;
use crate::renderer::Renderer;
use shipyard::prelude::*;
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

impl Renderer {
    /// gets the first found camera node 
    pub fn get_camera_node(&self) -> Option<Key> {
        let world = self.world.borrow();
        world.run::<(&CameraView, &CameraProjection), _, _>(|(views, projs)| {
            (&views, &projs).iter().with_id().map(|(id, _, _)| id).next()
        })
    }
    /// if no node is provided then the first camera node will be used 
    pub fn update_camera_projection(&mut self, node: Option<Key>, projection:&[f64]) {
        let node = if node.is_none() { self.get_camera_node() } else { node };
        if let Some(node) = node {
            let world = self.world.borrow_mut();
            let proj = world.run::<&mut CameraProjection, _, _>(|mut projs| {
                if let Some(proj) = (&mut projs).get(node).iter_mut().next() {
                    proj.0.as_mut().copy_from_slice(projection);
                } 
            });
        }
    }
    /// if no node is provided then the first camera node will be used 
    pub fn update_camera_view(&mut self, node: Option<Key>) {
        let node = if node.is_none() { self.get_camera_node() } else { node };
        if let Some(node) = node {
            let world = self.world.borrow_mut();
            world.run::<(&mut CameraView, &LocalTransform), _, _>(|(mut views, local_mats)| {
                if let Some((view, local_mat)) = (&mut views, &local_mats).get(node).iter_mut().next() {
                    let view = &mut view.0;
                    let local_mat = &local_mat.0;
                    view.copy_from_slice(local_mat.as_ref());
                    view.invert_mut().unwrap();
                } 
            });
        }
    }

    /// if no node is provided then the first camera node will be used 
    pub(crate) fn update_camera_ubo(&mut self, node:Option<Key>) {
        let node = if node.is_none() { self.get_camera_node() } else { node };
        if let Some(node) = node {
            let world = self.world.borrow_mut();
            let webgl = self.webgl.borrow_mut();

            world.run::<(&CameraView, &CameraProjection), _, _>(|(views, projs)| {
                if let Some((view, proj)) = (&views, &projs).get(node).iter_mut().next() {
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
}