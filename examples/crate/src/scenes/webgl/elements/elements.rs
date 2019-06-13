use awsm::webgl::{Id, GlToggle, BufferTarget, AttributeOptions, DataType, ClearBufferMask, WebGlRenderer, UniformMatrixData, UniformData, BeginMode};
use awsm::helpers::*;
use awsm::camera;
use awsm::tick::{start_raf_ticker_timestamp, Timestamp};
use std::rc::Rc; 
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{Window, Document, HtmlElement};
use crate::scenes::webgl::common::{start_webgl, create_unit_box_buffers, N_BOX_ELEMENTS}; 

//TODO - match https://github.com/dakom/pure3d-typescript/blob/master/examples/src/app/scenes/basic/box/box-vao-renderer/Box-Vao-Renderer.ts
//
pub fn start(window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {

    let state = Rc::new(RefCell::new(State::new()));

    let on_resize = {
        let state = Rc::clone(&state);
        move |width:u32, height: u32| {
            let mut state = state.borrow_mut();
            state.camera_width = width.into();
            state.camera_height = height.into();
        }
    };


    let webgl_renderer = start_webgl(window, document, body, on_resize)?;
    let webgl_renderer_clone = Rc::clone(&webgl_renderer);

    let mut webgl_renderer = webgl_renderer.borrow_mut();

    let _program_id = webgl_renderer.compile_program(
        include_str!("shaders/elements-vertex.glsl"),
        include_str!("shaders/elements-fragment.glsl")
    )?;

    let buffer_ids = create_unit_box_buffers(&mut webgl_renderer)?;

    let _ = start_raf_ticker_timestamp({
        let state = Rc::clone(&state);
        move |timestamp:Timestamp| {
            let mut state = state.borrow_mut();
            state.update(timestamp.time);
            render(&state, buffer_ids, &mut webgl_renderer_clone.borrow_mut()).unwrap();
        }
    })?;

    Ok(())
}

struct State {
    //mutable for each tick
    pub pos: Point,
    pub volume: Volume,
    pub camera_width: f64,
    pub camera_height: f64,
}

impl State {
    pub fn new() -> Self {
        Self {
            pos: Point{x: 0.0, y: 0.0, z: 0.0},
            volume: Volume{width: 400.0, height: 100.0, depth: 50.0},
            camera_width: 0.0,
            camera_height: 0.0,
        }

    }

    pub fn update(self:&mut Self, _time_stamp:f64) {

    }
}

type BufferIds = (Id, Id, Id);

fn render(state:&State, buffer_ids: BufferIds, webgl_renderer:&mut WebGlRenderer) -> Result<(), JsValue> {
    let mut scratch_matrix:[f32;16] = [0.0;16]; 
    let mut projection_matrix:[f32;16] = [0.0;16]; 
    let mut eye_matrix:[f32;16] = [0.0;16]; 
    let mut scale_matrix:[f32;16] = [0.0;16];
    let mut mvp_matrix:[f32;16] = [0.0;16];
    let mut camera_matrix:[f32;16] = [0.0;16];
    let mut color_vec:[f32;4] = [0.0;4];
    let State {pos, volume, ..} = state;
    let (geom_id, colors_id, elements_id) = buffer_ids;


    webgl_renderer.toggle(GlToggle::DepthTest, true);

    //scale
    write_scale_matrix(volume.width, volume.height, volume.depth, &mut scale_matrix);
    webgl_renderer.set_uniform_matrix_name("u_size", UniformMatrixData::Float4(&scale_matrix))?;

    //camera
    let projection = camera::write_persp(
        std::f64::consts::PI / 2.0, 
        state.camera_width / state.camera_height, 
        1.0, 
        3000.0, 
        &mut projection_matrix
    );

    let eye = camera::look_at(
        &vec![1000.0, 500.0, 1000.0],
        &vec![0.0, 0.0, 0.0],
        &vec![0.0, 1.0, 0.0],
        &mut eye_matrix
    );

    write_multiply_matrix(&projection_matrix, &eye_matrix, &mut camera_matrix); 
    //write_multiply_matrix(&eye_matrix, &projection_matrix, &mut camera_matrix); 
    //model-view-projection
    write_position_matrix(pos.x, pos.y, pos.z, &mut scratch_matrix);
    write_multiply_matrix(&camera_matrix, &scratch_matrix, &mut mvp_matrix); 
    webgl_renderer.set_uniform_matrix_name("u_modelViewProjection", UniformMatrixData::Float4(&mvp_matrix))?;

    webgl_renderer.activate_buffer(elements_id, BufferTarget::ElementArrayBuffer)?;

    webgl_renderer.activate_buffer_for_attribute_name(
        geom_id, 
        BufferTarget::ArrayBuffer,
        "a_vertex",
        &AttributeOptions::new(3, DataType::Float)
    )?;

    webgl_renderer.activate_buffer_for_attribute_name(
        colors_id, 
        BufferTarget::ArrayBuffer,
        "a_color",
        &AttributeOptions::new(3, DataType::Float)
    )?;


    //draw!
    webgl_renderer.clear(&[ClearBufferMask::ColorBufferBit, ClearBufferMask::DepthBufferBit]);
    webgl_renderer.draw_elements(BeginMode::Triangles, N_BOX_ELEMENTS, DataType::UnsignedByte, 0);
    Ok(())
}
