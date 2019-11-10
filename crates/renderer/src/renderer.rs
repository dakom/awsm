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
use awsm_web::errors::Error;
use crate::gltf::GltfResource;
use crate::gltf::renderer as gltf_renderer;
use log::info;
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

    pub fn upload_gltf(&mut self, resource:&GltfResource) -> Result<(), Error> {
        let mut webgl = self.webgl.borrow_mut();
        let GltfResource {gltf, buffers, images} = resource;

        gltf_renderer::buffer_view::upload_buffer_views(&mut webgl, &gltf, buffers)?;
        //gltf_renderer::accessors::upload_accessors(&mut webgl, &gltf, buffers)?;
        info!("adding data to gltf for {} nodes", gltf.nodes().len());

        Ok(())
    }

    pub fn set_scene_from_gltf(&mut self, gltf:&gltf::Document) {
    }
}
