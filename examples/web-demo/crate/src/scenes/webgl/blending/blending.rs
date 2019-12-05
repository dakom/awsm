use crate::router::get_static_href;
use crate::scenes::webgl::common::*;
use crate::start_webgl;
use awsm_web::loaders::fetch;
use awsm_web::webgl::{
    BeginMode, BlendFactor, ClearBufferMask, GlToggle, Id, PixelFormat, SimpleTextureOptions,
    TextureTarget, WebGlTextureSource,
};
use log::info;
use nalgebra::{Matrix4, Point2, Vector3};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Document, HtmlElement, Window};

struct State {
    //mutable for each tick
    pub pos: Point2<f64>,
    pub area: Area,
    pub camera_width: f64,
    pub camera_height: f64,
    pub program_id: Option<Id>,
    pub bottom_texture_id: Option<Id>,
    pub top_texture_id: Option<Id>,
}

impl State {
    pub fn new() -> Self {
        Self {
            pos: Point2::new(500.0, 500.0),
            area: Area::new(300.0, 100.0),
            camera_width: 0.0,
            camera_height: 0.0,
            program_id: None,
            bottom_texture_id: None,
            top_texture_id: None,
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

    let document_clone = document.clone();

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
                    include_str!("shaders/blending-vertex.glsl"),
                    include_str!("shaders/blending-fragment.glsl"),
                )?;

                state.borrow_mut().program_id = Some(program_id);

                let _buffer_id = create_and_assign_unit_quad_buffer(&mut webgl_renderer)?;

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

                    info!("{}x{}", state_obj.area.width, state_obj.area.height);

                    let canvas_image = generate_canvas_image(
                        &document_clone,
                        state_obj.area.width as u32,
                        state_obj.area.height as u32,
                        "rgba(255, 0, 0, .5)",
                    )?;

                    let texture_id = webgl_renderer.create_texture()?;
                    webgl_renderer.assign_simple_texture(
                        texture_id,
                        TextureTarget::Texture2d,
                        &SimpleTextureOptions {
                            pixel_format: PixelFormat::Rgba,
                            ..SimpleTextureOptions::default()
                        },
                        &WebGlTextureSource::CanvasElement(&canvas_image),
                    )?;
                    state_obj.top_texture_id = Some(texture_id);

                    let (width, height) = webgl_renderer.current_size();

                    reposition(&mut state_obj, width, height);

                    let texture_id = webgl_renderer.create_texture()?;

                    webgl_renderer.assign_simple_texture(
                        texture_id,
                        TextureTarget::Texture2d,
                        &SimpleTextureOptions {
                            pixel_format: PixelFormat::Rgba,
                            ..SimpleTextureOptions::default()
                        },
                        &WebGlTextureSource::ImageElement(&img),
                    )?;
                    state_obj.bottom_texture_id = Some(texture_id);

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

                reposition(&mut state, width, height);
            }
        },
        {
            let state = Rc::clone(&state);
            move |_time, webgl_renderer| {
                webgl_renderer.clear(&[
                    ClearBufferMask::ColorBufferBit,
                    ClearBufferMask::DepthBufferBit,
                ]);
                let mut render = |is_top: bool| {
                    let state = state.borrow();
                    let State {
                        pos,
                        area,
                        camera_width,
                        camera_height,
                        program_id,
                        top_texture_id,
                        bottom_texture_id,
                    } = *state;

                    let (pos_z, target_texture_id) = match is_top {
                        true => (0.0, top_texture_id.unwrap()),
                        false => (-100.0, bottom_texture_id.unwrap()),
                    };

                    webgl_renderer
                        .activate_program(program_id.unwrap())
                        .unwrap();
                    webgl_renderer.set_depth_mask(false);
                    webgl_renderer.toggle(GlToggle::Blend, true);
                    webgl_renderer
                        .set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);

                    //enable texture
                    webgl_renderer
                        .activate_texture_for_sampler(target_texture_id, "u_sampler")
                        .unwrap();

                    //Build our matrices (must cast to f32)
                    let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(
                        area.width as f32,
                        area.height as f32,
                        0.0,
                    ));
                    let camera_mat = Matrix4::new_orthographic(
                        0.0,
                        camera_width as f32,
                        0.0,
                        camera_height as f32,
                        0.0,
                        1000.0,
                    );
                    let model_mat =
                        Matrix4::new_translation(&Vector3::new(pos.x as f32, pos.y as f32, pos_z));
                    let mvp_mat = camera_mat * model_mat;

                    //Upload them to the GPU
                    webgl_renderer
                        .upload_uniform_mat_4("u_size", &scaling_mat.as_slice())
                        .unwrap();
                    webgl_renderer
                        .upload_uniform_mat_4("u_modelViewProjection", &mvp_mat.as_slice())
                        .unwrap();

                    //draw!
                    webgl_renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);
                };

                render(false);
                render(true);
            }
        }
    )
}

fn reposition(state: &mut State, width: u32, height: u32) {
    state.pos = Point2::new(
        ((width as f64) - state.area.width) / 2.0,
        ((height as f64) - state.area.height) / 2.0,
    );
}
