use crate::router::get_static_href;
use crate::scenes::webgl::common::*;
use crate::start_webgl;
use awsm_web::errors::Error;
use awsm_web::loaders::fetch;
use awsm_web::webgl::{
    AttributeOptions, BeginMode, ClearBufferMask, DataType, Id, SimpleTextureOptions,
    TextureCubeFace, TextureTarget, VertexArray, WebGlTextureSource,
};
use futures::future::{try_join_all};
use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::HtmlImageElement;
use web_sys::{Document, HtmlElement, Window};

struct State {
    //mutable for each tick
    pub camera_width: f64,
    pub camera_height: f64,
    pub program_id: Option<Id>,
    pub texture_id: Option<Id>,
    pub vao_id: Option<Id>,
}

impl State {
    pub fn new() -> Self {
        Self {
            camera_width: 0.0,
            camera_height: 0.0,
            program_id: None,
            texture_id: None,
            vao_id: None,
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

                if let WebGlVersion::One = version {}

                let program_id = webgl_renderer.compile_program(
                    include_str!("shaders/texture_cube-vertex.glsl"),
                    include_str!("shaders/texture_cube-fragment.glsl"),
                )?;

                state.borrow_mut().program_id = Some(program_id);

                let texture_id = webgl_renderer.create_texture()?;
                state.borrow_mut().texture_id = Some(texture_id);

                let future = async move {
                    let mut webgl_renderer = webgl_renderer_clone.borrow_mut();

                    let futures = vec![
                        fetch::image(&get_static_href("environment/env_x_pos.jpg")),
                        fetch::image(&get_static_href("environment/env_x_neg.jpg")),
                        fetch::image(&get_static_href("environment/env_y_pos.jpg")),
                        fetch::image(&get_static_href("environment/env_y_neg.jpg")),
                        fetch::image(&get_static_href("environment/env_z_pos.jpg")),
                        fetch::image(&get_static_href("environment/env_z_neg.jpg")),
                    ];

                    let images: Vec<HtmlImageElement> = try_join_all(futures).await?;

                    let (_width, _height) = webgl_renderer.current_size();

                    images
                        .into_iter()
                        .enumerate()
                        .map(|(i, img)| {
                            let face = match i {
                                0 => TextureCubeFace::PositiveX,
                                1 => TextureCubeFace::NegativeX,
                                2 => TextureCubeFace::PositiveY,
                                3 => TextureCubeFace::NegativeY,
                                4 => TextureCubeFace::PositiveZ,
                                5 => TextureCubeFace::NegativeZ,
                                _ => panic!("internal error going past index for cubemap!"),
                            };

                            webgl_renderer.assign_simple_texture(
                                texture_id,
                                TextureTarget::CubeMap,
                                &SimpleTextureOptions {
                                    flip_y: Some(false),
                                    cube_face: Some(face),
                                    ..SimpleTextureOptions::default()
                                },
                                &WebGlTextureSource::ImageElement(&img),
                            )
                        })
                        .collect::<Result<_, Error>>()?;

                    let vao_id = webgl_renderer.create_vertex_array()?;

                    let (geom_id, _colors_id, elements_id) =
                        create_unit_box_buffers(&mut webgl_renderer)?;

                    webgl_renderer.assign_vertex_array(
                        vao_id,
                        Some(elements_id),
                        &vec![VertexArray {
                            attribute_name: "a_vertex",
                            buffer_id: geom_id,
                            opts: &AttributeOptions::new(3, DataType::Float),
                        }],
                    )?;
                    state.borrow_mut().vao_id = Some(vao_id);

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
            move |_time, webgl_renderer| {
                let state = state.borrow();
                let State {
                    camera_width: _,
                    camera_height: _,
                    program_id,
                    vao_id,
                    texture_id,
                } = *state;

                webgl_renderer
                    .activate_program(program_id.unwrap())
                    .unwrap();

                //enable texture
                webgl_renderer
                    .activate_texture_for_sampler(texture_id.unwrap(), "u_sampler")
                    .unwrap();

                //Build our matrices (must cast to f32)
                // Our camera looks toward the point (1.0, 0.0, 0.0).
                // It is located at (0.0, 0.0, 1.0).
                let eye = Point3::new(0.0, 0.0, 0.0);
                let target = Point3::new(0.0, 0.0, 1000.0);
                let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

                // A perspective projection.
                let projection = Perspective3::new(
                    state.camera_width as f32 / state.camera_height as f32,
                    std::f32::consts::PI / 2.0,
                    1.0,
                    3000.0,
                );

                let mvp_mat = projection.to_homogeneous() * (view.to_homogeneous());

                let size = 900.0f32;
                let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(size, size, size));

                //Upload them to the GPU
                webgl_renderer
                    .upload_uniform_mat_4("u_modelViewProjection", &mvp_mat.as_slice())
                    .unwrap();
                webgl_renderer
                    .upload_uniform_mat_4("u_size", &scaling_mat.as_slice())
                    .unwrap();

                //activate buffers
                webgl_renderer
                    .activate_vertex_array(vao_id.unwrap())
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
        }
    )
}
