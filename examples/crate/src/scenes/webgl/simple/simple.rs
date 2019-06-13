use awsm::webgl::{ClearBufferMask, WebGlRenderer, UniformMatrixData, UniformData, BeginMode};
use awsm::helpers::*;
use awsm::camera::{write_ortho};
use awsm::tick::{start_raf_ticker_timestamp, Timestamp};
use std::rc::Rc; 
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{Window, Document, HtmlElement};
use crate::scenes::webgl::common::{start_webgl, create_and_assign_unit_quad_buffer}; 

pub fn start(window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {

    let state = Rc::new(RefCell::new(State::new()));

    let on_resize = {
        let state = Rc::clone(&state);
        move |width:u32, height: u32| {
            let mut state = state.borrow_mut();
            state.camera_width = width.into();
            state.camera_height = height.into();
            reposition(&mut state, width, height);
        }
    };


    let webgl_renderer = start_webgl(window, document, body, on_resize)?;
    let webgl_renderer_clone = Rc::clone(&webgl_renderer);

    let mut webgl_renderer = webgl_renderer.borrow_mut();

    let _program_id = webgl_renderer.compile_program(
        include_str!("shaders/simple-vertex.glsl"),
        include_str!("shaders/simple-fragment.glsl")
    )?;

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

fn reposition(state:&mut State, width: u32, height: u32) {

    state.pos = Point{
        x: ((width as f64) - state.area.width) / 2.0,
        y: ((height as f64) - state.area.height) / 2.0,
        z: 0.0
    };
}
struct State {
    //mutable for each tick
    pub pos: Point,
    pub area: Area,
    pub color: Color,
    pub camera_width: f64,
    pub camera_height: f64,
    pub direction: f64,
}

impl State {
    pub fn new() -> Self {
        Self {
            pos: Point{x: 500.0, y: 500.0, z: 0.0},
            area: Area{width: 300.0, height: 100.0},
            color: Color::new(1.0, 1.0, 0.0, 1.0),
            camera_width: 0.0,
            camera_height: 0.0,
            direction: 0.05, 
        }

    }

    pub fn update(self:&mut Self, _time_stamp:f64) {
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
}

fn render(state:&State, webgl_renderer:&mut WebGlRenderer) -> Result<(), JsValue> {
    let mut scratch_matrix:[f32;16] = [0.0;16]; 
    let mut scale_matrix:[f32;16] = [0.0;16];
    let mut mvp_matrix:[f32;16] = [0.0;16];
    let mut camera_matrix:[f32;16] = [0.0;16];
    let mut color_vec:[f32;4] = [0.0;4];
    let State {pos, area, color, ..} = state;


    //scale
    write_scale_matrix(area.width, area.height, 1.0, &mut scale_matrix);
    webgl_renderer.set_uniform_matrix_name("u_size", UniformMatrixData::Float4(&scale_matrix))?;

    //camera
    write_ortho(0.0, state.camera_width, 0.0, state.camera_height, 0.0, 1.0, &mut camera_matrix);

    //model-view-projection
    write_position_matrix(pos.x, pos.y, 0.0, &mut scratch_matrix);
    write_multiply_matrix(&camera_matrix, &scratch_matrix, &mut mvp_matrix); 
    webgl_renderer.set_uniform_matrix_name("u_modelViewProjection", UniformMatrixData::Float4(&mvp_matrix))?;

    //color
    color.write_to_v32_4(&mut color_vec);
    webgl_renderer.set_uniform_name("u_color", UniformData::Float4(&color_vec))?;

    //draw!
    webgl_renderer.clear(&[ClearBufferMask::ColorBufferBit, ClearBufferMask::DepthBufferBit]);
    webgl_renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);

    Ok(())
}
