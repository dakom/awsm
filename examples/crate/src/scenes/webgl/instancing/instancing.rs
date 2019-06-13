use awsm::webgl::{ClearBufferMask, SimpleTextureOptions, WebGlTextureSource, PixelFormat, Id, BufferTarget, BufferUsage, WebGlRenderer, AttributeOptions, DataType, UniformMatrixData, BeginMode};
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
use crate::scenes::webgl::common::{start_webgl, create_and_assign_unit_quad_buffer}; 
use log::{info};

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
        include_str!("shaders/instancing-vertex.glsl"),
        include_str!("shaders/instancing-fragment.glsl")
    )?;

    let _buffer_id = create_and_assign_unit_quad_buffer(&mut webgl_renderer)?;

    let texture_id = webgl_renderer.create_texture()?;

    let instance_pos_buffer_id = webgl_renderer.create_buffer()?; 

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

                let center = Point{
                    x: ((state_obj.camera_width as f64) - state_obj.area.width) / 2.0,
                    y: ((state_obj.camera_height as f64) - state_obj.area.height) / 2.0,
                    z: 0.0
                };

                for (pos, _) in state_obj.positions.iter_mut() {
                    pos.x = center.x; 
                    pos.y = center.y; 
                };

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
                        let mut state = state.borrow_mut();
                        for (pos, vel) in state.positions.iter_mut() {
                            pos.x += vel.x;
                            pos.y += vel.y;
                        }
                        let mut webgl_renderer = webgl_renderer_raf.borrow_mut();
                        render(&state, instance_pos_buffer_id, &mut webgl_renderer).unwrap();
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


struct State {
    //mutable for each tick
    pub positions: Vec<(Point, Point)>,
    pub area: Area,
    pub camera_width: f64,
    pub camera_height: f64,
}


impl State {
    pub fn new() -> Self {
        Self {
            positions: vec![
                (Point{x: 500.0, y: 500.0, z: 0.0}, Point{x: 1.0, y: 1.0, z: 0.0}),
                (Point{x: 800.0, y: 800.0, z: 0.0}, Point{x: -1.0, y: -1.0, z: 0.0}),
            ],
            area: Area{width: 300.0, height: 100.0},
            camera_width: 0.0,
            camera_height: 0.0,
        }

    }

}

fn render(state:&State, instance_pos_buffer_id: Id, webgl_renderer:&mut WebGlRenderer) -> Result<(), JsValue> {
    let mut scale_matrix:[f32;16] = [0.0;16];
    let mut camera_matrix:[f32;16] = [0.0;16];
    let State {positions, area, ..} = state;


    //scale
    write_scale_matrix(area.width, area.height, 1.0, &mut scale_matrix);
    webgl_renderer.set_uniform_matrix_name("u_size", UniformMatrixData::Float4(&scale_matrix))?;

    //camera
    write_ortho(0.0, state.camera_width, 0.0, state.camera_height, 0.0, 1.0, &mut camera_matrix);
    webgl_renderer.set_uniform_matrix_name("u_camera", UniformMatrixData::Float4(&camera_matrix))?;


    //upload our buffer for instancing
    //it's a big data move but we're doing it all at once.
    //without instancing we'd be doing separate draw calls and setting the uniform each time
    //there's almost definitely faster ways of creating the pos_data but this is clear for demo purposes
    let mut pos_data:Vec<f32> = Vec::new();
    for (pos, _) in positions.iter() {
        pos_data.push(pos.x as f32);
        pos_data.push(pos.y as f32);
    }

    //info!("{:#?}", scale_matrix);

    let loc = webgl_renderer.get_attribute_location("a_position")?;
    webgl_renderer.upload_buffer_f32(
        instance_pos_buffer_id, 
        &pos_data, 
        BufferTarget::ArrayBuffer, 
        BufferUsage::StaticDraw
    )?;

    webgl_renderer.activate_attribute_loc(
        loc,
        &AttributeOptions::new(2, DataType::Float)
    );


    //draw! (gotta clear first due to the extension needing mutability)
    webgl_renderer.clear(&[ClearBufferMask::ColorBufferBit, ClearBufferMask::DepthBufferBit]);

    webgl_renderer.vertex_attrib_divisor(loc, 1)?;
    webgl_renderer.draw_arrays_instanced(BeginMode::TriangleStrip, 0, 4, 2)?;

    Ok(())
}
