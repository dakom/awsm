use awsm::webgl::{ClearBufferMask, SimpleTextureOptions, WebGlTextureSource, PixelFormat, WebGlRenderer, UniformMatrixData, BeginMode};
use awsm::helpers::*;
use awsm::loaders::{image};
use crate::router::{get_static_href};
use awsm::camera::{write_ortho};
use awsm::tick::{start_raf_ticker_timestamp, Timestamp};
use std::rc::Rc; 
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::futures_0_3::{future_to_promise};
use web_sys::{Window, Document, HtmlElement};
use crate::scenes::webgl::common::{start_webgl, create_unit_quad_buffer}; 
use log::{info};

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
        include_str!("shaders/texture-vertex.glsl"),
        include_str!("shaders/texture-fragment.glsl")
    )?;

    let _buffer_id = create_unit_quad_buffer(&mut webgl_renderer)?;

    let texture_id = webgl_renderer.create_texture()?;

    let future = async move {
        let webgl_renderer = webgl_renderer_clone.borrow_mut();

        let href = get_static_href("smiley.svg");
        info!("loading image! {}", href);
        match image::fetch_image(href).await {
            Ok(img) => {

                let mut state_obj = state.borrow_mut();
                state_obj.area = Area{
                    width: img.natural_width().into(),
                    height: img.natural_height().into() 
                };

                let (width, height) = webgl_renderer.current_size();

                reposition(&mut state_obj, width, height);

                webgl_renderer.assign_simple_texture_2d(
                    texture_id, 
                    &SimpleTextureOptions{
                        pixel_format: PixelFormat::Rgba,
                        ..SimpleTextureOptions::default()
                    },
                    &WebGlTextureSource::ImageElement(&img)
                )?;

                let _cancel = start_raf_ticker_timestamp({
                    let state = Rc::clone(&state);
                    let webgl_renderer_raf = Rc::clone(&webgl_renderer_clone);
                    move |_timestamp:Timestamp| {
                        let state = state.borrow_mut();
                        let mut webgl_renderer = webgl_renderer_raf.borrow_mut();
                        render(&state, &mut webgl_renderer).unwrap();
                    }
                })?;
                Ok(JsValue::null())
            },

            Err(err) => {
                info!("error!");
                Err(err.into())
            }
        }
    };

    //we don't handle errors here because they are exceptions
    //hope you're running in an environment where uncaught rejects/exceptions are reported!
    future_to_promise(future);

    Ok(())
}

fn reposition(state:&mut State, width: u32, height: u32) {

    state.pos = Point{
        x: ((width as f64) - state.area.width) / 2.0,
        y: ((height as f64) - state.area.height) / 2.0
    };
}

struct State {
    //mutable for each tick
    pub pos: Point,
    pub area: Area,
    pub camera_width: f64,
    pub camera_height: f64,
}

impl State {
    pub fn new() -> Self {
        Self {
            pos: Point{x: 500.0, y: 500.0},
            area: Area{width: 300.0, height: 100.0},
            camera_width: 0.0,
            camera_height: 0.0,
        }

    }

}

fn render(state:&State, webgl_renderer:&mut WebGlRenderer) -> Result<(), JsValue> {
    let mut scratch_matrix:[f32;16] = [0.0;16]; 
    let mut scale_matrix:[f32;16] = [0.0;16];
    let mut mvp_matrix:[f32;16] = [0.0;16];
    let mut camera_matrix:[f32;16] = [0.0;16];
    let State {pos, area, ..} = state;


    //scale
    write_scale_matrix(area.width, area.height, 1.0, &mut scale_matrix);
    webgl_renderer.set_uniform_matrix_name("u_size", UniformMatrixData::Float4(&scale_matrix))?;

    //camera
    write_ortho(0.0, state.camera_width, 0.0, state.camera_height, 0.0, 1.0, &mut camera_matrix);

    //model-view-projection
    write_position_matrix(pos.x, pos.y, 0.0, &mut scratch_matrix);
    write_multiply_matrix(&camera_matrix, &scratch_matrix, &mut mvp_matrix); 
    webgl_renderer.set_uniform_matrix_name("u_modelViewProjection", UniformMatrixData::Float4(&mvp_matrix))?;

    //draw!
    webgl_renderer.clear(&[ClearBufferMask::ColorBufferBit, ClearBufferMask::DepthBufferBit]);
    webgl_renderer.draw_arrays(BeginMode::TriangleStrip as u32, 0, 4);

    Ok(())
}
