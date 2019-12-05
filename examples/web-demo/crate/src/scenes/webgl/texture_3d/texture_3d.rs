use crate::router::get_static_href;
use crate::scenes::webgl::common::*;
use awsm_web::data::TypedData;
use awsm_web::loaders::fetch;
use awsm_web::webgl::PartialWebGlTextures;
use awsm_web::webgl::{
    BeginMode, ClearBufferMask, DataType, Id, PixelFormat, SimpleTextureOptions, TextureMagFilter,
    TextureMinFilter, TextureOptions, TextureTarget, TextureWrapMode, TextureWrapTarget,
    WebGlTextureSource,
};
use gloo_events::EventListener;
use log::info;
use lut_parser::CubeLut;
use nalgebra::{Matrix4, Point2, Vector3};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{Document, HtmlElement, WebGl2RenderingContext, Window};

struct State {
    //mutable for each tick
    pub lut_enabled: bool,
    pub pos: Point2<f64>,
    pub area: Area,
    pub camera_width: f64,
    pub camera_height: f64,
    pub program_id: Option<Id>,
    pub diffuse_texture_id: Option<Id>,
    pub lut_texture_id: Option<Id>,
}

impl State {
    pub fn new() -> Self {
        Self {
            lut_enabled: false,
            pos: Point2::new(500.0, 500.0),
            area: Area::new(300.0, 100.0),
            camera_width: 0.0,
            camera_height: 0.0,
            program_id: None,
            diffuse_texture_id: None,
            lut_texture_id: None,
        }
    }
}

pub fn start(window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {
    let state = Rc::new(RefCell::new(State::new()));

    let document_clone = document.clone();
    let body_clone = body.clone();
    start_webgl_2(
        window,
        document_clone,
        body_clone,
        {
            let state = Rc::clone(&state);
            move |webgl_renderer, on_ready| {
                let webgl_renderer_clone = Rc::clone(&webgl_renderer);

                let mut webgl_renderer = webgl_renderer.borrow_mut();

                let program_id = webgl_renderer.compile_program(
                    include_str!("shaders/texture_3d-vertex.glsl"),
                    include_str!("shaders/texture_3d-fragment.glsl"),
                )?;

                state.borrow_mut().program_id = Some(program_id);

                let _buffer_id = create_and_assign_unit_quad_buffer(&mut webgl_renderer)?;

                let diffuse_texture_id = webgl_renderer.create_texture()?;
                state.borrow_mut().diffuse_texture_id = Some(diffuse_texture_id);

                let lut_texture_id = webgl_renderer.create_texture()?;
                state.borrow_mut().lut_texture_id = Some(lut_texture_id);

                let future = async move {
                    let mut webgl_renderer = webgl_renderer_clone.borrow_mut();

                    let href = get_static_href("photo.jpg");
                    info!("loading image! {}", href);
                    let img = fetch::image(&href).await?;
                    let href = get_static_href("LUT.cube");
                    let txt: String = fetch::text(&href).await?;
                    let lut =
                        CubeLut::<f32>::from_str(&txt).map_err(|_| "couldn't parse cube file")?;

                    let mut state_obj = state.borrow_mut();
                    state_obj.area = Area {
                        width: img.natural_width().into(),
                        height: img.natural_height().into(),
                    };

                    let (width, height) = webgl_renderer.current_size();

                    reposition(&mut state_obj, width, height);

                    webgl_renderer.upload_uniform_fval("u_lut_size", lut.size as f32)?;
                    let data_obj: js_sys::Object = TypedData::new(&lut.flatten_data()).into();

                    webgl_renderer.assign_texture(
                        lut_texture_id,
                        TextureTarget::Texture3d,
                        &TextureOptions {
                            internal_format: PixelFormat::Rgb32f,
                            data_format: PixelFormat::Rgb,
                            data_type: DataType::Float,
                            cube_face: None,
                        },
                        Some(|gl: &WebGl2RenderingContext| {
                            let bind_target = TextureTarget::Texture3d;

                            gl.awsm_texture_set_wrap(
                                bind_target,
                                TextureWrapTarget::S,
                                TextureWrapMode::Repeat,
                            );
                            gl.awsm_texture_set_wrap(
                                bind_target,
                                TextureWrapTarget::T,
                                TextureWrapMode::Repeat,
                            );
                            gl.awsm_texture_set_wrap(
                                bind_target,
                                TextureWrapTarget::R,
                                TextureWrapMode::Repeat,
                            );

                            gl.awsm_texture_set_min_filter(bind_target, TextureMinFilter::Nearest);
                            gl.awsm_texture_set_mag_filter(bind_target, TextureMagFilter::Nearest);
                        }),
                        &WebGlTextureSource::ArrayBufferView(&data_obj, 32, 32, 32),
                    )?;

                    webgl_renderer.assign_simple_texture(
                        diffuse_texture_id,
                        TextureTarget::Texture2d,
                        &SimpleTextureOptions {
                            pixel_format: PixelFormat::Rgba,
                            ..SimpleTextureOptions::default()
                        },
                        &WebGlTextureSource::ImageElement(&img),
                    )?;

                    let button = create_button(state_obj.lut_enabled, &document, &body)?;

                    let my_cb = {
                        let button = button.clone();
                        let state = Rc::clone(&state);
                        move |_: &_| {
                            let mut state = state.borrow_mut();
                            state.lut_enabled = !state.lut_enabled;
                            set_button_label(&button, state.lut_enabled);
                        }
                    };

                    EventListener::new(&button, "click", my_cb).forget();

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
                let state = state.borrow();

                let State {
                    pos,
                    area,
                    camera_width,
                    camera_height,
                    program_id,
                    diffuse_texture_id,
                    lut_texture_id,
                    lut_enabled,
                } = *state;

                webgl_renderer
                    .activate_program(program_id.unwrap())
                    .unwrap();

                //enable texture
                webgl_renderer
                    .activate_texture_for_sampler(diffuse_texture_id.unwrap(), "u_diffuse_sampler")
                    .unwrap();
                webgl_renderer
                    .activate_texture_for_sampler(lut_texture_id.unwrap(), "u_lut_sampler")
                    .unwrap();

                //Build our matrices (must cast to f32)
                let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(
                    area.width as f32,
                    area.height as f32,
                    0.0f32,
                ));
                let projection_mat = Matrix4::new_orthographic(
                    0.0,
                    camera_width as f32,
                    0.0,
                    camera_height as f32,
                    0.0,
                    1.0,
                );
                let model_mat =
                    Matrix4::new_translation(&Vector3::new(pos.x as f32, pos.y as f32, 0.0));
                let mvp_mat = projection_mat * model_mat;

                //Upload them to the GPU
                webgl_renderer
                    .upload_uniform_mat_4("u_size", &scaling_mat.as_slice())
                    .unwrap();
                webgl_renderer
                    .upload_uniform_mat_4("u_modelViewProjection", &mvp_mat.as_slice())
                    .unwrap();

                //set uniform for toggle
                webgl_renderer
                    .upload_uniform_uval("u_lut_enabled", lut_enabled as u32)
                    .unwrap();

                //draw!
                webgl_renderer.clear(&[
                    ClearBufferMask::ColorBufferBit,
                    ClearBufferMask::DepthBufferBit,
                ]);
                webgl_renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);
            }
        },
    )
}

fn reposition(state: &mut State, width: u32, height: u32) {
    state.pos = Point2::new(
        ((width as f64) - state.area.width) / 2.0,
        ((height as f64) - state.area.height) / 2.0,
    );
}

fn create_button(
    lut_enabled: bool,
    document: &Document,
    container: &HtmlElement,
) -> Result<HtmlElement, JsValue> {
    let item: HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_class_name("button demo-button");
    set_button_label(&item, lut_enabled);
    container.append_child(&item)?;
    Ok(item)
}

fn set_button_label(button: &HtmlElement, lut_enabled: bool) {
    if lut_enabled {
        button.set_text_content(Some("DISABLE"));
    } else {
        button.set_text_content(Some("ENABLE"));
    }
}
