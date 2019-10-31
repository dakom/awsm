use std::rc::Rc;
use std::cell::RefCell;
use futures::future::{self, FutureExt, TryFutureExt};
use futures::Future;
use awsm_web::webgl::{
    WebGl2Renderer
};
use awsm_web::errors::Error;

pub struct Renderer {
    pub webgl:Rc<RefCell<WebGl2Renderer>>
}

impl Renderer {
    pub fn new(webgl:Rc<RefCell<WebGl2Renderer>>, width: u32, height: u32) -> Self {
        let mut ret = Self{webgl};

        ret.resize(width, height);

        ret
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.webgl.borrow_mut().resize(width, height);
    }

    pub fn render(&mut self, _interpolation:Option<f64>) {
    }

    pub fn animate(&mut self, _delta:f64) {
    }

    pub fn load_gltf(&mut self, gltf_path:&str) -> impl Future<Output = Result<(), Error>> {
        async {
            Ok(())
        }
    }
}
