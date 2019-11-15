use crate::scenes::webgl::common::*;
use crate::start_webgl;
use awsm_web::webgl::{BeginMode, ClearBufferMask, Id};
use nalgebra::{Matrix4, Point2, Vector3};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, Window};

struct State {
    //mutable for each tick
    pub pos: Point2<f64>,
    pub area: Area,
    pub color: Color,
    pub camera_width: f64,
    pub camera_height: f64,
    pub direction: f64,
    pub program_id: Option<Id>,
}

impl State {
    pub fn new() -> Self {
        Self {
            pos: Point2::new(500.0, 500.0),
            area: Area::new(300.0, 100.0),
            color: Color::new(1.0, 1.0, 0.0, 1.0),
            camera_width: 0.0,
            camera_height: 0.0,
            direction: 0.05,
            program_id: None,
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
                let _webgl_renderer_clone = Rc::clone(&webgl_renderer);

                let mut webgl_renderer = webgl_renderer.borrow_mut();

                let program_id = webgl_renderer.compile_program(
                    include_str!("shaders/simple-vertex.glsl"),
                    include_str!("shaders/simple-fragment.glsl"),
                )?;

                state.borrow_mut().program_id = Some(program_id);
                let _buffer_id = create_and_assign_unit_quad_buffer(&mut webgl_renderer)?;
                on_ready();
                Ok(())
            }
        },
        {
            let state = Rc::clone(&state);
            move |width: u32, height: u32| {
                let mut state = state.borrow_mut();
                state.resize(width.into(), height.into());
            }
        },
        {
            let state = Rc::clone(&state);

            move |time, webgl_renderer| {
                {
                    let mut state = state.borrow_mut();
                    state.update(time);
                }

                let state = state.borrow();

                let State {
                    pos,
                    area,
                    color,
                    camera_width,
                    camera_height,
                    program_id,
                    ..
                } = *state;

                webgl_renderer
                    .activate_program(program_id.unwrap())
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
                    1.0,
                );
                let model_mat =
                    Matrix4::new_translation(&Vector3::new(pos.x as f32, pos.y as f32, 0.0));
                let mvp_mat = camera_mat * model_mat;

                //Upload them to the GPU
                webgl_renderer
                    .upload_uniform_mat_4("u_size", &scaling_mat.as_slice())
                    .unwrap();
                webgl_renderer
                    .upload_uniform_mat_4("u_modelViewProjection", &mvp_mat.as_slice())
                    .unwrap();

                let color_values = color.values();
                let color_values = (
                    color_values[0] as f32,
                    color_values[1] as f32,
                    color_values[2] as f32,
                    color_values[3] as f32,
                );
                webgl_renderer
                    .upload_uniform_fvals_4("u_color", color_values)
                    .unwrap();

                //draw!
                webgl_renderer.clear(&[
                    ClearBufferMask::ColorBufferBit,
                    ClearBufferMask::DepthBufferBit,
                ]);
                webgl_renderer.draw_arrays(BeginMode::TriangleStrip, 0, 4);
            }
        }
    )
}

impl State {
    pub fn update(&mut self, _time_stamp: f64) {
        let color = &mut self.color;
        let direction = &mut (self.direction);
        color.r += *direction;
        if *direction > 0.0 {
            if color.r > 1.0 {
                color.r = 1.0;
                *direction *= -1.0;
            }
        } else {
            if color.r < 0.0 {
                color.r = 0.0;
                *direction *= -1.0;
            }
        }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.camera_width = width;
        self.camera_height = height;

        self.pos = Point2::new(
            (width as f64 - self.area.width) / 2.0,
            (height as f64 - self.area.height) / 2.0,
        );
    }
}
