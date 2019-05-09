use crate::rust::helpers::matrix::*;
use super::quad_data::*;
use crate::rust::scenes::scene::{Scene};
use awsm_webgl::enums::{BeginMode};
use awsm_webgl::renderer::*;
use awsm_webgl::errors::*;
use awsm_webgl::uniforms::{UniformData, UniformMatrixData};
use std::rc::Rc;
use std::cell::RefCell;
use futures::future::{Future, result};
use web_sys::{console};
use wasm_bindgen::prelude::*;

pub struct QuadScene <'a>{
    webgl_renderer: Rc<RefCell<WebGlRenderer<'a>>>, 
    camera_matrix:[f32;16],
    instance_data:QuadInstanceData,
    render_data:QuadRenderData,
}

impl <'a> QuadScene <'a>{
    pub fn new(webgl_renderer:Rc<RefCell<WebGlRenderer>>) -> impl Future<Item = Box<QuadScene>, Error = Error> {

        let instance_data = QuadInstanceData::new();

        //this must all be in its own scope since we can't take ownership of
        //webgl_renderer while the borrow is still active
        //
        let render_data_result = {
            webgl_renderer.try_borrow_mut()
                .map_err(|s| Error::from(s.to_string()))
                .and_then(|mut webgl_renderer_ref| {
                    QuadRenderData::new(&mut webgl_renderer_ref)
                })
        };


        result(render_data_result)
            .map(|render_data| {
                Box::new(QuadScene{
                    webgl_renderer,
                    camera_matrix: [0.0;16],
                    instance_data,
                    render_data,
                })
            })
    }
}

impl <'a> Drop for QuadScene<'a> {
    fn drop(self:&mut Self) {
        console::log_1(&JsValue::from_str("DROPPED QUAD SCENE!"));
    }
}

impl <'a> Scene for QuadScene<'a> {

    fn id(self:&Self) -> &str {
        "quad"
    }

    fn tick(self:&mut Self, time_stamp:f64, _delta_time:f64) -> Result<(), Error> {
        self.instance_data.update(time_stamp);
        self.render_data.update(&self.camera_matrix, &self.instance_data);
        
        let mut webgl_renderer_ref = self.webgl_renderer.try_borrow_mut().map_err(|e| e.to_string())?;
        self.render(&mut webgl_renderer_ref)?;
        Ok(())
    }

    fn resize(self:&mut Self, window_width: u32, window_height: u32) -> Result<(), Error> {
        //Note - we could also get it from self.webgl_renderer.borrow_mut etc.
        write_ortho(0.0, window_width as f64, 0.0, window_height as f64, 0.0, 1.0, &mut self.camera_matrix);
        Ok(())
    }
}


impl <'a> WebGlRender for QuadScene<'a> {
    fn render(self: &Self, webgl_renderer:&mut WebGlRenderer) -> Result<(), Error> {
        let render_data = &self.render_data; 

        //scale
        webgl_renderer.set_uniform_matrix_name("u_size", UniformMatrixData::Float4(&render_data.scale_matrix))?;

        //model-view-projection
        webgl_renderer.set_uniform_matrix_name("u_modelViewProjection", UniformMatrixData::Float4(&render_data.mvp_matrix))?;

        //color
        webgl_renderer.set_uniform_name("u_color", UniformData::Float4(&render_data.color_vec))?;
       
        //draw!
        webgl_renderer.draw_arrays(BeginMode::TriangleStrip as u32, 0, 4);

        Ok(())
    }
}
