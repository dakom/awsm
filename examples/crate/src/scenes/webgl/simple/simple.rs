use awsm::webgl::{Id, UniformLocation, ClearBufferMask, WebGlRenderer, Uniform, BeginMode};
use awsm::tick::{start_raf_ticker_timestamp, Timestamp};
use std::rc::Rc; 
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{Window, Document, HtmlElement};
use crate::scenes::webgl::common::{start_webgl, create_and_assign_unit_quad_buffer}; 
use nalgebra::{Matrix4, Vector3, Vector4, Point2};
use crate::scenes::webgl::common::datatypes::*;

struct State {
    //mutable for each tick
    pub pos: Point2<f64>,
    pub area: Area,
    pub color: Color,
    pub camera_width: f64,
    pub camera_height: f64,
    pub direction: f64,
    pub program_id: Option<Id>,
}

impl State {
    pub fn new() -> Self {
        Self {
            pos: Point2::new(500.0, 500.0),
            area: Area::new(300.0, 100.0),
            color: Color::new(1.0, 1.0, 0.0, 1.0),
            camera_width: 0.0,
            camera_height: 0.0,
            direction: 0.05, 
            program_id: None
        }

    }
}
pub fn start(window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {

    let state = Rc::new(RefCell::new(State::new()));

    let on_resize = {
        let state = Rc::clone(&state);
        move |width:u32, height: u32| {
            let mut state = state.borrow_mut();
            state.resize(width.into(), height.into());
        }
    };


    let webgl_renderer = start_webgl(window, document, body, on_resize)?;
    let webgl_renderer_clone = Rc::clone(&webgl_renderer);

    let mut webgl_renderer = webgl_renderer.borrow_mut();

    let program_id = webgl_renderer.compile_program(
        include_str!("shaders/simple-vertex.glsl"),
        include_str!("shaders/simple-fragment.glsl")
    )?;

    state.borrow_mut().program_id = Some(program_id);
    let _buffer_id = create_and_assign_unit_quad_buffer(&mut webgl_renderer)?;

    let _ = start_raf_ticker_timestamp({
        let state = Rc::clone(&state);
        move |timestamp:Timestamp| {
            let mut state = state.borrow_mut();
            state.update(timestamp.time);
            render(&state, &mut webgl_renderer_clone.borrow_mut()).unwrap();
        }
    })?;

    Ok(())
}

impl State {
    pub fn update(&mut self, _time_stamp:f64) {
        let color = &mut self.color;
        let direction = &mut (self.direction);
        color.r += *direction;
        if *direction > 0.0 {
            if color.r > 1.0 {
                color.r = 1.0;
                *direction *= -1.0;
            }
        } else {
            if color.r < 0.0 {
                color.r = 0.0;
                *direction *= -1.0;
            }
        }

    }

    pub fn resize(&mut self, width:f64, height:f64) {
        self.camera_width = width;
        self.camera_height = height;

        self.pos = Point2::new(
            (width as f64 - self.area.width) / 2.0,
            (height as f64 - self.area.height) / 2.0,
        );
    }
}

fn render(state:&State, webgl_renderer:&mut WebGlRenderer) -> Result<(), JsValue> {
    let State {pos, area, color, camera_width, camera_height, program_id, ..} = state;

    webgl_renderer.activate_program(program_id.unwrap());

    //Build our matrices (must cast to f32)
    let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(area.width as f32, area.height as f32, 0.0));
    let camera_mat = Matrix4::new_orthographic(0.0, *camera_width as f32, 0.0, *camera_height as f32, 0.0, 1.0);
    let model_mat = Matrix4::new_translation(&Vector3::new(pos.x as f32, pos.y as f32, 0.0));
    let mvp_mat = camera_mat * model_mat;

    //Upload them to the GPU
    webgl_renderer.upload_uniform(&UniformLocation::Name("u_size"), &Uniform::Matrix4(&scaling_mat.as_slice()))?;
    webgl_renderer.upload_uniform(&UniformLocation::Name("u_modelViewProjection"), &Uniform::Matrix4(&mvp_mat.as_slice()))?;
    webgl_renderer.upload_uniform(&UniformLocation::Name("u_color"), &Uniform::Slice4(&color.values()))?;

    //draw!
    webgl_renderer.clear(&[ClearBufferMask::ColorBufferBit, ClearBufferMask::DepthBufferBit]);
    webgl_renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);


    Ok(())
}
