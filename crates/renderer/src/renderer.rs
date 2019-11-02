use std::rc::Rc;
use std::cell::RefCell;
use futures::future::{self, FutureExt, TryFutureExt};
use futures::Future;
use awsm_web::webgl::{
    WebGl2Renderer,
    ClearBufferMask
};
use awsm_web::errors::Error;
use crate::gltf::GltfResource;

pub struct Renderer {
    //This is Rc<RefCell> because other renderers might want to also own the context
    //for example a 2d renderer which gets passed to JS for whatever reason and must be 'static
    //There is almost no performance impact since it's only borrowed at the top of the core functions 
    //- not deep in the iterators
    pub webgl:Rc<RefCell<WebGl2Renderer>>
}

impl Renderer {
    pub fn new(webgl:Rc<RefCell<WebGl2Renderer>>, width: u32, height: u32) -> Self {
        let mut ret = Self{webgl};

        ret.resize(width, height);

        ret
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let mut webgl = self.webgl.borrow_mut();
        webgl.resize(width, height);
    }

    pub fn render(&mut self, clear_first:bool, _interpolation:Option<f64>) {
        let mut webgl = self.webgl.borrow_mut();

        if(clear_first) {
            webgl.clear(&[
                ClearBufferMask::ColorBufferBit,
                ClearBufferMask::DepthBufferBit,
            ]);
        }
    }

    pub fn animate(&mut self, _delta:f64) {
        let mut webgl = self.webgl.borrow_mut();
    }

    pub fn add_gltf(&mut self, resource:&GltfResource) {
        let mut webgl = self.webgl.borrow_mut();
    }
}
