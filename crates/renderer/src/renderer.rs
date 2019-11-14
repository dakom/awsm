use std::rc::Rc;
use std::cell::RefCell;
use futures::future::{self, FutureExt, TryFutureExt};
use futures::Future;
use awsm_web::webgl::{
    WebGl2Renderer,
    ClearBufferMask,
    BufferData,
    BufferTarget,
    BufferUsage
};
use crate::errors::{Error, NativeError};
use crate::gltf::GltfResource;
use crate::components::register_components;
use log::info;
use crate::gltf::processor::{ProcessState, process_scene};

use shipyard::*;

pub struct Renderer {
    //This is Rc<RefCell> because other renderers might want to also own the context
    //for example a 2d renderer which gets passed to JS for whatever reason and must be 'static
    //There is almost no performance impact since it's only borrowed at the top of the core functions 
    //- not deep in the iterators
    pub webgl:Rc<RefCell<WebGl2Renderer>>,
    world: Rc<RefCell<World>>
}

impl Renderer {
    pub fn new(webgl:Rc<RefCell<WebGl2Renderer>>, world: Option<Rc<RefCell<World>>>, width: u32, height: u32) -> Self {
        let world = match world {
            Some(world) => world,
            None => Rc::new(RefCell::new(World::default()))
        };

        let mut ret = Self{webgl, world};

        {
            let mut world = ret.world.borrow_mut();
            register_components(&mut world);
        }

        ret.resize(width, height);

        ret
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let mut webgl = self.webgl.borrow_mut();
        webgl.resize(width, height);
    }

    pub fn clear(&mut self) {
        let mut webgl = self.webgl.borrow_mut();

        webgl.clear(&[
            ClearBufferMask::ColorBufferBit,
            ClearBufferMask::DepthBufferBit,
        ]);
    }

    pub fn render(&mut self, _interpolation:Option<f64>) {
        let mut webgl = self.webgl.borrow_mut();
    }

    pub fn animate(&mut self, _delta:f64) {
        let mut webgl = self.webgl.borrow_mut();
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

    pub fn set_scene_from_gltf(&mut self, gltf:&gltf::Document) {
    }
}
