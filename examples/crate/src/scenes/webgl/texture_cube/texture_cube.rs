use awsm::webgl::{Id, DataType, AttributeOptions, VertexArray, TextureCubeFace, ClearBufferMask, TextureTarget, SimpleTextureOptions, WebGlTextureSource, PixelFormat, BeginMode};
use crate::{WebGlRenderer};
use awsm::errors::{Error};
use web_sys::{HtmlImageElement};
use awsm::loaders::{fetch};
use futures::future::{Future, join_all, try_join_all};
use crate::router::{get_static_href};
use awsm::tick::{Timestamp, start_timestamp_loop};
use std::rc::Rc; 
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::futures_0_3::{future_to_promise};
use web_sys::{Window, Document, HtmlElement};
use crate::scenes::webgl::common::{start_webgl, create_unit_box_buffers, N_BOX_ELEMENTS}; 
use crate::scenes::webgl::common::datatypes::*;
use nalgebra::{Matrix4, Vector3, Point2, Point3, Perspective3, Isometry3};
use log::{info};

struct State {
    //mutable for each tick
    pub camera_width: f64,
    pub camera_height: f64,
    pub program_id: Option<Id>,
    pub texture_id: Option<Id>,
    pub vao_id: Option<Id>
}

impl State {
    pub fn new() -> Self {
        Self {
            camera_width: 0.0,
            camera_height: 0.0,
            program_id: None,
            texture_id: None,
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
fn register_extensions(_webgl_renderer:&mut WebGlRenderer) -> Result<(), JsValue> {
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
        include_str!("shaders/texture_cube-vertex.glsl"),
        include_str!("shaders/texture_cube-fragment.glsl")
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

        let images:Vec<HtmlImageElement> = try_join_all(futures).await?;

        let (width, height) = webgl_renderer.current_size();

        images.into_iter().enumerate().map(|(i, img)| {
            let face = match i {
                0 => TextureCubeFace::PositiveX,
                1 => TextureCubeFace::NegativeX,
                2 => TextureCubeFace::PositiveY,
                3 => TextureCubeFace::NegativeY,
                4 => TextureCubeFace::PositiveZ,
                5 => TextureCubeFace::NegativeZ,
                _ => {
                    panic!("internal error going past index for cubemap!")
                }
            };

            webgl_renderer.assign_simple_texture(
                texture_id, 
                TextureTarget::CubeMap,
                &SimpleTextureOptions{
                    flip_y: Some(false),
                    cube_face: Some(face),
                    ..SimpleTextureOptions::default()
                },
                &WebGlTextureSource::ImageElement(&img)
            )
        }).collect::<Result<_, Error>>()?;

    
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
            ]
        )?;
        state.borrow_mut().vao_id = Some(vao_id);

        let _cancel = start_timestamp_loop({
            let state = Rc::clone(&state);
            let webgl_renderer_raf = Rc::clone(&webgl_renderer_clone);
            move |_timestamp:Timestamp| {
                let state = state.borrow_mut();
                let mut webgl_renderer = webgl_renderer_raf.borrow_mut();
                render(&state, &mut webgl_renderer).unwrap();
            }
        })?;
        Ok(JsValue::null())
    };

    //we don't handle errors here because they are exceptions
    //hope you're running in an environment where uncaught rejects/exceptions are reported!
    future_to_promise(future);

    Ok(())
}

fn render(state:&State, webgl_renderer:&mut WebGlRenderer) -> Result<(), JsValue> {
    let State {camera_width, camera_height, program_id, vao_id, texture_id} = state;

    webgl_renderer.activate_program(program_id.unwrap())?;

    //enable texture
    webgl_renderer.activate_texture_for_sampler(texture_id.unwrap(), "u_sampler")?;

    //Build our matrices (must cast to f32)
    // Our camera looks toward the point (1.0, 0.0, 0.0).
    // It is located at (0.0, 0.0, 1.0).
    let eye    = Point3::new(0.0, 0.0, 0.0);
    let target = Point3::new(0.0, 0.0, 1000.0);
    let view   = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

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
    webgl_renderer.upload_uniform_mat_4("u_modelViewProjection", &mvp_mat.as_slice())?;
    webgl_renderer.upload_uniform_mat_4("u_size", &scaling_mat.as_slice())?;

    //activate buffers
    webgl_renderer.activate_vertex_array(vao_id.unwrap())?;

    //draw!
    webgl_renderer.clear(&[ClearBufferMask::ColorBufferBit, ClearBufferMask::DepthBufferBit]);

    webgl_renderer.draw_elements(BeginMode::Triangles, N_BOX_ELEMENTS, DataType::UnsignedByte, 0);

    Ok(())
}
