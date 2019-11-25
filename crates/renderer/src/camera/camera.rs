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

/*

export const getPerspectiveProjection = (settings:Partial<PerspectiveCameraSettings>) => {
    const values = createMat4(); 
    const a = settings.aspectRatio === undefined && settings.canvas !== undefined
            ?   settings.canvas.clientWidth / settings.canvas.clientHeight
            :   settings.aspectRatio;
    const y = settings.yfov;
    const n = settings.znear;
    const f = settings.zfar; //if this is undefined, use infinite projection

    values[0] = 1/(a * Math.tan(.5 * y));
    values[5] = 1/(Math.tan(.5 * y));
    values[10] = f === undefined ? -1 : (f+n)/(n-f);
    values[11] = -1;
    values[14] = f === undefined ? (-2 * n) : (2 * f * n)/(n - f);


    return values; 
}
*/

impl Renderer {
    //if node is supplied, will update the camera at that node
    //otherwise, it is the first camera node found
    pub fn update_camera_projection(&mut self, node:Option<Key>, projection:&[f64]) {
        let world = self.world.borrow_mut();
        //TODO - if node is supplied, only use that 
        world.run::<(&mut CameraProjection), _>(|mut projs| {
            if let Some(proj) = projs.iter().next() {
                proj.0.as_mut().copy_from_slice(projection);
            }
        });
    }

    pub(crate) fn update_camera_ubo(&mut self) {
        let world = self.world.borrow_mut();
        let webgl = self.webgl.borrow_mut();

        //TODO - only do if marked as dirty
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