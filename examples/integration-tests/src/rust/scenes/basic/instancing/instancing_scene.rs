use crate::rust::helpers::matrix::*;
use super::instancing_data::*;
use super::instancing_render_data::*;
use crate::rust::scenes::scene::{Scene};
use awsm_webgl::enums::{BeginMode};
use awsm_webgl::renderer::*;
use awsm_webgl::errors::*;
use std::rc::Rc;
use std::cell::RefCell;
use futures::future::{Future, result};
use awsm_webgl::uniforms::{UniformMatrixData};

pub struct InstancingScene <'a> {
    webgl_renderer: Rc<RefCell<WebGlRenderer<'a>>>, 
    camera_matrix:[f32;16],
    instance_data:InstancingInstanceData,
    render_data:InstancingRenderData,
}

impl <'a> InstancingScene <'a> {
    pub fn new(webgl_renderer:Rc<RefCell<WebGlRenderer>>) -> impl Future<Item = Box<InstancingScene>, Error = Error> {
        InstancingInstanceData::new()
            .and_then(|instance_data| {
                //this must all be in its own scope since we can't take ownership of
                //webgl_renderer while the borrow is still active
                let render_data_result = {
                    webgl_renderer.try_borrow_mut()
                        .map_err(|s| Error::from(s.to_string()))
                        .and_then(|mut webgl_renderer_ref| {
                            InstancingRenderData::new(&mut webgl_renderer_ref, &instance_data)
                        })
                };

                result(render_data_result)
                    .map(|render_data| {
                        Box::new(InstancingScene{
                            webgl_renderer,
                            camera_matrix: [0.0;16],
                            instance_data,
                            render_data,
                        })
                    })
            })
    }
}

impl <'a> Scene for InstancingScene<'a> {
    fn id(self:&Self) -> &str {
        "instancing"
    }
    fn tick(self:&mut Self, _time_stamp:f64, delta_time:f64) -> Result<(), Error> {
        let mut webgl_renderer_ref = self.webgl_renderer.try_borrow_mut().map_err(|e| e.to_string())?;

        self.instance_data.update(delta_time);

        for bunny in &self.instance_data.bunnies {
            self.render_data.update(&self.camera_matrix, &self.instance_data.area, &bunny.pos);
            self.render(&mut webgl_renderer_ref)?;
        }
        

        Ok(())
    }

    fn resize(self:&mut Self, window_width: u32, window_height: u32) -> Result<(), Error> {
        //Note - we could also get it from self.webgl_renderer.borrow_mut etc.
        write_ortho(0.0, window_width as f64, 0.0, window_height as f64, 0.0, 1.0, &mut self.camera_matrix);
        Ok(())
    }

}


impl <'a>WebGlRender for InstancingScene<'a> {
    fn render(self: &Self, webgl_renderer:&mut WebGlRenderer) -> Result<(), Error> {
        let render_data = &self.render_data; 

        //TODO - the instancing shouldn't try and use the uniform location
        //Rather it should use the attribute location
        let loc = webgl_renderer.get_uniform_loc("u_modelViewProjection")?;
        //instancing
        let ext = webgl_renderer.get_extension_instanced_arrays()?;
        ext.vertex_attrib_divisor_angle(&loc, 1);
        //scale
        webgl_renderer.set_uniform_matrix_name("u_size", UniformMatrixData::Float4(&render_data.scale_matrix))?;

        //model-view-projection
        webgl_renderer.set_uniform_matrix_name("u_modelViewProjection", UniformMatrixData::Float4(&render_data.mvp_matrix))?;

        //draw!
        ext.draw_arrays_instanced_angle(BeginMode::TriangleStrip as u32, 0, 4, 1)?;
        //webgl_renderer.draw_arrays(BeginMode::TriangleStrip as u32, 0, 4);

        Ok(())
    }
}
