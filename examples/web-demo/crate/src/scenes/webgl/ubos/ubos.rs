use crate::scenes::webgl::common::*;
use awsm_web::errors::Error;
use awsm_web::webgl::{
    AttributeOptions, BeginMode, BufferData, BufferTarget, BufferUsage, ClearBufferMask, DataType,
    GlToggle, Id, VertexArray, WebGl2Renderer,
};
use log::info;
use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, Window};

/*
 * In order to test the various ways of setting ubo's
 * The model is fully set on init
 * The camera is updated every tick
 * The scale is partially updated every tick
 *
 */
struct State {
    //mutable for each tick
    pub pos: Point3<f64>,
    pub volume: Volume,
    pub camera_width: f64,
    pub camera_height: f64,
    pub program_id: Option<Id>,
    pub vao_id: Option<Id>,
    pub model_buffer_id: Option<Id>,
    pub camera_buffer_id: Option<Id>,
    pub scale_buffer_id: Option<Id>,
}

impl State {
    pub fn new() -> Self {
        Self {
            pos: Point3::new(0.0, 0.0, 0.0),
            volume: Volume::new(400.0, 100.0, 50.0),
            camera_width: 0.0,
            camera_height: 0.0,
            program_id: None,
            vao_id: None,
            model_buffer_id: None,
            camera_buffer_id: None,
            scale_buffer_id: None,
        }
    }
}

pub fn start(window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {
    let state = Rc::new(RefCell::new(State::new()));

    start_webgl_2(
        window,
        document,
        body,
        {
            let state = Rc::clone(&state);
            move |webgl_renderer, on_ready| {
                let _webgl_renderer_clone = Rc::clone(&webgl_renderer);

                let mut webgl_renderer = webgl_renderer.borrow_mut();

                //simple test that the global registery works
                //camera will be bound at 1 (due to being registered)
                //model will be bound at 3 (due to not being registered
                webgl_renderer.register_global_uniform_buffer("dog");
                webgl_renderer.register_global_uniform_buffer("camera");
                webgl_renderer.register_global_uniform_buffer("chair");

                let program_id = webgl_renderer.compile_program(
                    include_str!("shaders/ubos-vertex.glsl"),
                    include_str!("shaders/ubos-fragment.glsl"),
                )?;

                let mut state_obj = state.borrow_mut();

                state_obj.program_id = Some(program_id);

                let vao_id = webgl_renderer.create_vertex_array()?;

                let (geom_id, colors_id, elements_id) =
                    create_unit_box_buffers(&mut webgl_renderer)?;

                let camera_buffer_id = webgl_renderer.create_buffer()?;
                state_obj.camera_buffer_id = Some(camera_buffer_id);

                let model_buffer_id = webgl_renderer.create_buffer()?;
                state_obj.model_buffer_id = Some(model_buffer_id);

                set_model_buffer(&mut state_obj, &webgl_renderer)?;

                let scale_buffer_id = webgl_renderer.create_buffer()?;
                state_obj.scale_buffer_id = Some(scale_buffer_id);

                set_initial_scale_buffer(scale_buffer_id, &webgl_renderer)?;

                webgl_renderer.assign_vertex_array(
                    vao_id,
                    Some(elements_id),
                    &vec![
                        VertexArray {
                            attribute_name: "a_vertex",
                            buffer_id: geom_id,
                            opts: &AttributeOptions::new(3, DataType::Float),
                        },
                        VertexArray {
                            attribute_name: "a_color",
                            buffer_id: colors_id,
                            opts: &AttributeOptions::new(3, DataType::Float),
                        },
                    ],
                )?;

                state_obj.vao_id = Some(vao_id);

                on_ready();
                Ok(())
            }
        },
        {
            let state = Rc::clone(&state);
            move |width: u32, height: u32| {
                let mut state = state.borrow_mut();
                state.camera_width = width.into();
                state.camera_height = height.into();
            }
        },
        {
            let state = Rc::clone(&state);
            move |_time, webgl_renderer| {
                let mut state = state.borrow_mut();

                webgl_renderer
                    .activate_program(state.program_id.unwrap())
                    .unwrap();

                webgl_renderer.toggle(GlToggle::DepthTest, true);

                //will partially upload (but fully activate) the buffer
                update_scale_buffer(&mut state, webgl_renderer).unwrap();

                //will upload and activate buffer
                update_camera_buffer(&mut state, webgl_renderer).unwrap();

                //Model just needs to be activated so that it pulls from the buffer (which was set at init time)
                webgl_renderer
                    .activate_uniform_buffer(state.model_buffer_id.unwrap(), "model")
                    .unwrap();

                //activate VAO's
                webgl_renderer
                    .activate_vertex_array(state.vao_id.unwrap())
                    .unwrap();

                //draw!
                webgl_renderer.clear(&[
                    ClearBufferMask::ColorBufferBit,
                    ClearBufferMask::DepthBufferBit,
                ]);
                webgl_renderer.draw_elements(
                    BeginMode::Triangles,
                    N_BOX_ELEMENTS,
                    DataType::UnsignedByte,
                    0,
                );
            }
        },
    )
}

fn set_initial_scale_buffer(
    scale_buffer_id: Id,
    webgl_renderer: &WebGl2Renderer,
) -> Result<(), Error> {
    //Upload them to the GPU as a UBO
    let scale: [f32; 3] = [1.0; 3];

    //Just set it in a buffer, will be used at render time
    webgl_renderer.upload_buffer(
        scale_buffer_id,
        BufferData::new(
            &scale,
            BufferTarget::UniformBuffer,
            BufferUsage::DynamicDraw,
        ),
    )
}

fn update_scale_buffer(state: &State, webgl_renderer: &WebGl2Renderer) -> Result<(), Error> {
    let scale_y: [f32; 3] = [0.0, 3.0, 0.0];

    webgl_renderer.upload_buffer_sub_to_uniform_buffer_f32(
        "u_scale_y",
        "scale",
        state.scale_buffer_id.unwrap(),
        &scale_y[1..2],
    )
}

fn set_model_buffer(state: &State, webgl_renderer: &WebGl2Renderer) -> Result<(), Error> {
    let State {
        pos,
        volume,
        ..
    } = state;

    let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(
        volume.width as f32,
        volume.height as f32,
        volume.depth as f32,
    ));

    info!("{}", scaling_mat);

    let model_mat =
        Matrix4::new_translation(&Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32));

    //Upload them to the GPU as a UBO
    let model = vec![scaling_mat.as_slice(), model_mat.as_slice()].concat();

    //Just set it in a buffer, will be used at render time
    webgl_renderer.upload_buffer(
        state.model_buffer_id.unwrap(),
        BufferData::new(
            &model,
            BufferTarget::UniformBuffer,
            BufferUsage::DynamicDraw,
        ),
    )
}

fn update_camera_buffer(state: &State, webgl_renderer: &WebGl2Renderer) -> Result<(), Error> {
    // Our camera looks toward the point (1.0, 0.0, 0.0).
    // It is located at (0.0, 0.0, 1.0).
    let eye = Point3::new(1000.0, 500.0, 1000.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y()).to_homogeneous();

    // A perspective projection.
    let projection = Perspective3::new(
        state.camera_width as f32 / state.camera_height as f32,
        std::f32::consts::PI / 2.0,
        1.0,
        3000.0,
    )
    .to_homogeneous();

    let camera = vec![view.as_slice(), projection.as_slice()].concat();

    //will activate it too
    webgl_renderer.upload_buffer_to_uniform_buffer_f32(
        "camera",
        state.camera_buffer_id.unwrap(),
        &camera,
        BufferUsage::DynamicDraw,
    )
}
