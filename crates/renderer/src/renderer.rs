use std::rc::Rc;
use std::cell::RefCell;
use awsm_web::webgl::{ WebGl2Renderer, ClearBufferMask, BufferData, BufferTarget, BufferUsage, Id};
use crate::errors::{Error, NativeError};
use crate::gltf::loader::GltfResource;
use crate::components::*;
use crate::primitives::PrimitiveDraw;
use crate::gltf::processor::{ProcessState, process_scene};

use shipyard::*;

pub struct Renderer {
    //This is Rc<RefCell> because other renderers might want to also own the context
    //for example a 2d renderer which gets passed to JS for whatever reason and must be 'static
    //There is almost no performance impact since it's only borrowed at the top of the core functions 
    //- not deep in the iterators
    pub webgl:Rc<RefCell<WebGl2Renderer>>,
    pub world: Rc<RefCell<World>>,

    pub(crate) camera_buffer_id: Id,
}

impl Renderer {
    pub fn new(webgl:Rc<RefCell<WebGl2Renderer>>, world: Option<Rc<RefCell<World>>>, width: u32, height: u32) -> Result<Self, Error> {
        let world = match world {
            Some(world) => world,
            None => Rc::new(RefCell::new(World::default()))
        };

        let camera_buffer_id = webgl.borrow_mut().create_buffer()?;
        let mut ret = Self{webgl, world, camera_buffer_id};

        {
            let mut world = ret.world.borrow_mut();
            register_components(&mut world);
        }

        ret.resize(width, height);

        Ok(ret)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let mut webgl = self.webgl.borrow_mut();
        webgl.resize(width, height);
    }

    pub fn clear(&mut self) {
        let webgl = self.webgl.borrow_mut();

        webgl.clear(&[
            ClearBufferMask::ColorBufferBit,
            ClearBufferMask::DepthBufferBit,
        ]);
    }


    fn update_transforms(&mut self) {
        let mut webgl = self.webgl.borrow_mut();
        let world = self.world.borrow_mut();

        //Update all the LocalMatrices if translation, rotation, or scale changed
        //TODO - only update if marked as dirty
        world.run::<(&Translation, &Rotation, &Scale, &mut LocalMatrix), _>(|(translations, rotations, scales, local_matrices)| {
            for (translation, rotation, scale, mut local_matrix) in (translations, rotations, scales, local_matrices).iter() {
                let local_matrix = &mut local_matrix.0;
                let translation = &translation.0;
                let rotation = &rotation.0;
                let scale = &scale.0;
                local_matrix.from_trs_mut(translation, rotation, scale);
            }
        });

        //Update all the WorldMatrices if any ancestor changed 
        //TODO - only update if marked as dirty
        world.run::<(&Node, &LocalMatrix, &mut WorldMatrix), _>(|(nodes, local_matrices, world_matrices)| {
            let mut parent_matrix = Matrix4::default();
            let mut child_index = 0;
            for (node, local_matrix, world_matrix) in (nodes, local_matrices, world_matrices).iter() {
                let local_matrix = &local_matrix.0;
                let world_matrix = &mut world_matrix.0;

                world_matrix.copy_from(&parent_matrix);
                world_matrix.mul_mut(local_matrix);

                if child_index == node.n_children {
                    parent_matrix.copy_from(world_matrix);
                    child_index = 0;
                } else {
                    child_index += 1;
                }
            }
        });
    }

    pub fn render(&mut self, _interpolation:Option<f64>) {
        self.update_transforms();
        self.update_camera_ubo();

        let mut webgl = self.webgl.borrow_mut();
        let world = self.world.borrow_mut();

        world.run::<(&Primitive, &WorldMatrix), _>(|(primitives, model_matrices)| {
            for (primitive, model_matrix) in (primitives, model_matrices).iter() {
                let Primitive{shader_id, vao_id, draw_info} = primitive;

                webgl.activate_program(*shader_id).unwrap();
                webgl.activate_uniform_buffer(self.camera_buffer_id, "camera").unwrap();
                webgl.upload_uniform_mat_4("u_model", &model_matrix.0.to_vec_f32()).unwrap();
                webgl.activate_vertex_array(*vao_id).unwrap();

                match draw_info {
                    PrimitiveDraw::Elements(draw_mode, count, data_type, offset) => {
                        webgl.draw_elements(*draw_mode, *count, *data_type, *offset);
                        //log::info!("draw mode: {}, count: {}, offset: {}", *draw_mode as u32, *count, *offset);
                    },
                    PrimitiveDraw::Direct(draw_mode, count, offset) => {
                        webgl.draw_arrays(*draw_mode, *offset, *count);
                    }
                };
            }
        });
    }

    pub fn animate(&mut self, _delta:f64) {
        let _webgl = self.webgl.borrow_mut();
    }

    //The scene will be determined by the following in order of preference
    //1. scene in argument
    //2. default scene set in gltf
    //3. first in scenes array
    //if none of these exist, it's an error (not supporting gltf as asset library atm)
    pub fn upload_gltf(&mut self, resource:&GltfResource, scene:Option<gltf::scene::Scene>) -> Result<(), Error> {
        let mut webgl = self.webgl.borrow_mut();
        let mut world = self.world.borrow_mut();

        let scene = 
            scene.or(
                resource.gltf.default_scene().or(
                    resource.gltf.scenes().next()
                )
        ).ok_or(NativeError::SceneMissing)?;

        process_scene(ProcessState::new(resource,&mut world,&mut webgl), &scene)?;



            /*
            if let Some(mesh) = node.mesh() {
                mesh.primitives().any(|primitive| {

                    if primitive.indices().map(|acc| acc.index()).contains(&accessor_id) {
                        return true;
                    }
                    if primitive.attributes().any(|(_, attribute_accessor)| {
                        attribute_accessor.index() == accessor_id
                    }) {
                        return true;
                    }
                    if primitive.morph_targets().any(|morph_target| {
                        morph_target.positions().map(|acc| acc.index()).contains(&accessor_id) 
                            || morph_target.normals().map(|acc| acc.index()).contains(&accessor_id) 
                            || morph_target.tangents().map(|acc| acc.index()).contains(&accessor_id)
                    }) {
                        return true;
                    }

                    false
                })
            } else {
                false
            }
            */
        //let mut buffer_ids = gltf_renderer::buffer_view::upload_buffer_views(&mut webgl, &gltf, &buffers)?;
        //gltf_renderer::accessors::populate_accessors(&mut webgl, &mut world, &gltf, &mut buffer_ids, &buffers);
        //gltf_renderer::accessors::upload_accessors(&mut webgl, &gltf, buffers)?;

        Ok(())
    }

    pub fn set_scene_from_gltf(&mut self, _gltf:&gltf::Document) {
    }
}
