use crate::router::get_static_href;
use crate::scenes::webgl::common::*;
use crate::start_webgl;
use awsm_web::loaders::fetch;
use awsm_web::tick::{Timestamp, TimestampLoop};
use awsm_web::webgl::{
    AttributeOptions, BeginMode, BufferData, BufferTarget, BufferUsage, ClearBufferMask, DataType,
    Id, PixelFormat, SimpleTextureOptions, TextureTarget, WebGlTextureSource,
};
use log::info;
use nalgebra::{Matrix4, Point2, Vector3};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::futures_0_3::future_to_promise;
use web_sys::{Document, HtmlElement, Window};

struct State {
    //mutable for each tick
    pub positions: Vec<(Point2<f64>, Point2<f64>)>,
    pub area: Area,
    pub camera_width: f64,
    pub camera_height: f64,
    pub program_id: Option<Id>,
    pub texture_id: Option<Id>,
    pub instance_id: Option<Id>,
}

impl State {
    pub fn new() -> Self {
        Self {
            positions: vec![
                (Point2::new(500.0, 500.0), Point2::new(0.1, 0.1)),
                (Point2::new(800.0, 800.0), Point2::new(-0.1, -0.1)),
            ],
            area: Area::new(300.0, 100.0),
            camera_width: 0.0,
            camera_height: 0.0,
            program_id: None,
            texture_id: None,
            instance_id: None,
        }
    }
}

pub fn start(
    window: Window,
    document: Document,
    body: HtmlElement,
    version: WebGlVersion,
) -> Result<(), JsValue> {
    let state = Rc::new(RefCell::new(State::new()));

    start_webgl!(
        version,
        window,
        document,
        body,
        {
            let state = Rc::clone(&state);
            move |webgl_renderer, on_ready| {
                let webgl_renderer_clone = Rc::clone(&webgl_renderer);

                let mut webgl_renderer = webgl_renderer.borrow_mut();

                let program_id = webgl_renderer.compile_program(
                    include_str!("shaders/instancing-vertex.glsl"),
                    include_str!("shaders/instancing-fragment.glsl"),
                )?;

                state.borrow_mut().program_id = Some(program_id);
                let _buffer_id = create_and_assign_unit_quad_buffer(&mut webgl_renderer)?;

                let texture_id = webgl_renderer.create_texture()?;

                state.borrow_mut().texture_id = Some(texture_id);

                let instance_id = webgl_renderer.create_buffer()?;

                state.borrow_mut().instance_id = Some(instance_id);

                let future = async move {
                    let mut webgl_renderer = webgl_renderer_clone.borrow_mut();

                    let href = get_static_href("smiley.svg");
                    info!("loading image! {}", href);
                    let img = fetch::image(&href).await?;

                    let mut state_obj = state.borrow_mut();
                    state_obj.area = Area {
                        width: img.natural_width().into(),
                        height: img.natural_height().into(),
                    };

                    let center = Point2::new(
                        ((state_obj.camera_width as f64) - state_obj.area.width) / 2.0,
                        ((state_obj.camera_height as f64) - state_obj.area.height) / 2.0,
                    );

                    for (pos, _) in state_obj.positions.iter_mut() {
                        pos.x = center.x;
                        pos.y = center.y;
                    }

                    webgl_renderer.assign_simple_texture(
                        texture_id,
                        TextureTarget::Texture2d,
                        &SimpleTextureOptions {
                            pixel_format: PixelFormat::Rgba,
                            ..SimpleTextureOptions::default()
                        },
                        &WebGlTextureSource::ImageElement(&img),
                    )?;

                    on_ready();
                    Ok(JsValue::null())
                };

                //we don't handle errors here because they are exceptions
                //hope you're running in an environment where uncaught rejects/exceptions are reported!
                future_to_promise(future);

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
            move |time, webgl_renderer| {
                {
                    let mut state = state.borrow_mut();
                    for (pos, vel) in state.positions.iter_mut() {
                        pos.x += vel.x * time;
                        pos.y += vel.y * time;
                    }
                }
                let state = state.borrow();
                let State {
                    positions,
                    area,
                    camera_width,
                    camera_height,
                    program_id,
                    texture_id,
                    instance_id,
                } = &*state;

                //draw! (gotta clear first due to the extension needing mutability)
                webgl_renderer.clear(&[
                    ClearBufferMask::ColorBufferBit,
                    ClearBufferMask::DepthBufferBit,
                ]);

                webgl_renderer
                    .activate_program(program_id.unwrap())
                    .unwrap();

                //enable texture
                webgl_renderer
                    .activate_texture_for_sampler(texture_id.unwrap(), "u_sampler")
                    .unwrap();

                //Build our matrices (must cast to f32)
                let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(
                    area.width as f32,
                    area.height as f32,
                    0.0,
                ));
                let camera_mat = Matrix4::new_orthographic(
                    0.0,
                    *camera_width as f32,
                    0.0,
                    *camera_height as f32,
                    0.0,
                    1.0,
                );

                //Upload them to the GPU
                webgl_renderer
                    .upload_uniform_mat_4("u_size", &scaling_mat.as_slice())
                    .unwrap();
                webgl_renderer
                    .upload_uniform_mat_4("u_camera", &camera_mat.as_slice())
                    .unwrap();

                //upload our buffer for instancing
                //there's almost definitely faster ways of creating the pos_data but this is clear for demo purposes
                let mut pos_data: Vec<f32> = Vec::new();
                for (pos, _) in positions.iter() {
                    pos_data.push(pos.x as f32);
                    pos_data.push(pos.y as f32);
                }

                //need the location for the attrib_divisor below
                let loc = webgl_renderer
                    .get_attribute_location_value("a_position")
                    .unwrap();
                webgl_renderer
                    .upload_buffer(
                        instance_id.unwrap(),
                        BufferData::new(
                            &pos_data,
                            BufferTarget::ArrayBuffer,
                            BufferUsage::StaticDraw,
                        ),
                    )
                    .unwrap();

                webgl_renderer
                    .activate_attribute_loc(loc, &AttributeOptions::new(2, DataType::Float));

                webgl_renderer.vertex_attrib_divisor(loc, 1).unwrap();
                webgl_renderer
                    .draw_arrays_instanced(BeginMode::TriangleStrip, 0, 4, 2)
                    .unwrap();
            }
        }
    )
}
