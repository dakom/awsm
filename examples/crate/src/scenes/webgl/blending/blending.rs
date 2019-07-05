use awsm::webgl::{Id, GlToggle, BlendFactor, ClearBufferMask,SimpleTextureOptions, WebGlTextureSource, PixelFormat, BeginMode};
use crate::{WebGlRenderer};
use awsm::loaders::{fetch};
use crate::router::{get_static_href};
use awsm::tick::{start_raf_ticker_timestamp, Timestamp};
use std::rc::Rc; 
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::futures_0_3::{future_to_promise};
use web_sys::{Window, Document, HtmlElement};
use crate::scenes::webgl::common::{generate_canvas_image, start_webgl, create_and_assign_unit_quad_buffer}; 
use crate::scenes::webgl::common::datatypes::*;
use nalgebra::{Matrix4, Vector3, Point2};
use log::{info};

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


    let webgl_renderer = start_webgl(window, document.clone(), body, on_resize)?;
    let webgl_renderer_clone = Rc::clone(&webgl_renderer);

    let mut webgl_renderer = webgl_renderer.borrow_mut();


    let program_id = webgl_renderer.compile_program(
        include_str!("shaders/blending-vertex.glsl"),
        include_str!("shaders/blending-fragment.glsl")
    )?;

    state.borrow_mut().program_id = Some(program_id);

    let _buffer_id = create_and_assign_unit_quad_buffer(&mut webgl_renderer)?;


    let future = async move {
        let mut webgl_renderer = webgl_renderer_clone.borrow_mut();

        let href = get_static_href("smiley.svg");
        info!("loading image! {}", href);
        let img = fetch::image(&href).await?;


        let mut state_obj = state.borrow_mut();
        state_obj.area = Area{
            width: img.natural_width().into(),
            height: img.natural_height().into() 
        };

        info!("{}x{}", state_obj.area.width, state_obj.area.height);

        let canvas_image = generate_canvas_image(&document, state_obj.area.width as u32, state_obj.area.height as u32, "rgba(255, 0, 0, .5)")?;

        let texture_id = webgl_renderer.create_texture()?;
        webgl_renderer.assign_simple_texture(
            texture_id, 
            &SimpleTextureOptions{
                pixel_format: PixelFormat::Rgba,
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::CanvasElement(&canvas_image)
        )?;
        state_obj.top_texture_id = Some(texture_id);

        let (width, height) = webgl_renderer.current_size();

        reposition(&mut state_obj, width, height);

        let texture_id = webgl_renderer.create_texture()?;

        webgl_renderer.assign_simple_texture(
            texture_id, 
            &SimpleTextureOptions{
                pixel_format: PixelFormat::Rgba,
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::ImageElement(&img)
        )?;
        state_obj.bottom_texture_id = Some(texture_id);

        let _cancel = start_raf_ticker_timestamp({
            let state = Rc::clone(&state);
            let webgl_renderer_raf = Rc::clone(&webgl_renderer_clone);
            move |_timestamp:Timestamp| {
                let state = state.borrow_mut();
                let mut webgl_renderer = webgl_renderer_raf.borrow_mut();
                webgl_renderer.clear(&[ClearBufferMask::ColorBufferBit, ClearBufferMask::DepthBufferBit]);
                render(&state, &mut webgl_renderer, false).unwrap();
                render(&state, &mut webgl_renderer, true).unwrap();
            }
        })?;
        Ok(JsValue::null())
    };

    //we don't handle errors here because they are exceptions
    //hope you're running in an environment where uncaught rejects/exceptions are reported!
    future_to_promise(future);

    Ok(())
}

fn reposition(state:&mut State, width: u32, height: u32) {

    state.pos = Point2::new(
        ((width as f64) - state.area.width) / 2.0,
        ((height as f64) - state.area.height) / 2.0,
    );
}


fn render(state:&State, webgl_renderer:&mut WebGlRenderer, is_top: bool) -> Result<(), JsValue> {
    let State {pos, area, camera_width, camera_height, program_id, top_texture_id, bottom_texture_id} = state;

    let (pos_z, target_texture_id) = match is_top {
        true => (0.0, top_texture_id.unwrap()),
        false => (-100.0, bottom_texture_id.unwrap()),
    };

    webgl_renderer.activate_program(program_id.unwrap())?;
    webgl_renderer.set_depth_mask(false); 
    webgl_renderer.toggle(GlToggle::Blend, true);
    webgl_renderer.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);

    //enable texture
    webgl_renderer.activate_texture_for_sampler(target_texture_id, "u_sampler")?;

    //Build our matrices (must cast to f32)
    let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(area.width as f32, area.height as f32, 0.0));
    let camera_mat = Matrix4::new_orthographic(0.0, *camera_width as f32, 0.0, *camera_height as f32, 0.0, 1000.0);
    let model_mat = Matrix4::new_translation(&Vector3::new(pos.x as f32, pos.y as f32, pos_z));
    let mvp_mat = camera_mat * model_mat;

    //Upload them to the GPU
    webgl_renderer.upload_uniform_mat_4("u_size", &scaling_mat.as_slice())?;
    webgl_renderer.upload_uniform_mat_4("u_modelViewProjection", &mvp_mat.as_slice())?;

    //draw!
    webgl_renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);


    Ok(())
}
