use awsm::webgl::{VertexArray, Id, GlToggle, BufferTarget, AttributeOptions, DataType, ClearBufferMask, WebGlRenderer, BeginMode};
use awsm::tick::{start_raf_ticker_timestamp, Timestamp};
use awsm::errors::{Error};
use std::rc::Rc; 
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{Window, Document, HtmlElement};
use crate::scenes::webgl::common::{start_webgl, create_unit_box_buffers, N_BOX_ELEMENTS}; 
use crate::scenes::webgl::common::datatypes::*;
use nalgebra::{Matrix4, Vector3, Vector4, Point3, Perspective3, Isometry3};

type BufferIds = (Id, Id, Id);
struct State {
    //mutable for each tick
    pub pos: Point3<f64>,
    pub volume: Volume,
    pub camera_width: f64,
    pub camera_height: f64,
    pub program_id: Option<Id>,
    pub vao_id: Option<Id>
}

impl State {
    pub fn new() -> Self {
        Self {
            pos: Point3::new(0.0, 0.0, 0.0),
            volume: Volume::new(400.0, 100.0, 50.0),
            camera_width: 0.0,
            camera_height: 0.0,
            program_id: None,
            vao_id: None
        }

    }
}

#[cfg(feature = "webgl_1")]
fn register_extensions(webgl_renderer:&mut WebGlRenderer) -> Result<(), JsValue> {
    webgl_renderer.register_extension_vertex_array()
        .map_err(|err| err.into())
        .map(|_| ())
}
#[cfg(feature = "webgl_2")]
fn register_extensions(webgl_renderer:&mut WebGlRenderer) -> Result<(), JsValue> {
    Ok(())
}
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

    register_extensions(&mut webgl_renderer)?;

    let program_id = webgl_renderer.compile_program(
        include_str!("shaders/vaos-vertex.glsl"),
        include_str!("shaders/vaos-fragment.glsl")
    )?;

    state.borrow_mut().program_id = Some(program_id);

    let vao_id = webgl_renderer.create_vertex_array()?;

    let (geom_id, colors_id, elements_id) = create_unit_box_buffers(&mut webgl_renderer)?;

    webgl_renderer.assign_vertex_array(
        vao_id,
        Some(elements_id),
        &vec![
            VertexArray{
                attribute_name: "a_vertex",
                buffer_id: geom_id,
                opts: &AttributeOptions::new(3, DataType::Float)
            },

            VertexArray{
                attribute_name: "a_color",
                buffer_id: colors_id,
                opts: &AttributeOptions::new(3, DataType::Float)
            }
        ]
    )?;

    state.borrow_mut().vao_id = Some(vao_id);

    let _ = start_raf_ticker_timestamp({
        let state = Rc::clone(&state);
        move |_timestamp:Timestamp| {
            let mut state = state.borrow_mut();
            render(&state, &mut webgl_renderer_clone.borrow_mut()).unwrap();
        }
    })?;

    Ok(())
}



fn render(state:&State, webgl_renderer:&mut WebGlRenderer) -> Result<(), JsValue> {
    let State {pos, volume, camera_width, camera_height, program_id, vao_id} = state;


    webgl_renderer.activate_program(program_id.unwrap());

    webgl_renderer.toggle(GlToggle::DepthTest, true);

    //Build our matrices (must cast to f32)
    let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(volume.width as f32, volume.height as f32, volume.depth as f32));


    // Our camera looks toward the point (1.0, 0.0, 0.0).
    // It is located at (0.0, 0.0, 1.0).
    let eye    = Point3::new(1000.0, 500.0, 1000.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let view   = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

    // A perspective projection.
    let projection = Perspective3::new(
        state.camera_width as f32 / state.camera_height as f32, 
        std::f32::consts::PI / 2.0, 
        1.0, 
        3000.0, 
    );

    let model_mat = Matrix4::new_translation(&Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32));
    let mvp_mat = projection.to_homogeneous() * (view.to_homogeneous() * model_mat);

    //Upload them to the GPU
    webgl_renderer.upload_uniform_mat_4("u_size", scaling_mat.as_slice())?;
    webgl_renderer.upload_uniform_mat_4("u_modelViewProjection", mvp_mat.as_slice())?;


    //activate buffers
    webgl_renderer.activate_vertex_array(vao_id.unwrap());

    //draw!
    webgl_renderer.clear(&[ClearBufferMask::ColorBufferBit, ClearBufferMask::DepthBufferBit]);
    webgl_renderer.draw_elements(BeginMode::Triangles, N_BOX_ELEMENTS, DataType::UnsignedByte, 0);

    Ok(())
}
