use awsm::webgl::{WebGlRenderer};
use awsm::helpers::*;
use awsm::window;
use wasm_bindgen::prelude::*;
use web_sys::{Window, Document, HtmlElement, HtmlCanvasElement, WebGl2RenderingContext};
use crate::scenes::webgl::common::*; 

struct State {
    pub pos: Point,
    pub area: Area,
    pub color: Color,
    pub direction: f64,
}

impl State {
    pub fn new() -> Self {
        Self {
            pos: Point{x: 500.0, y: 500.0},
            area: Area{width: 300.0, height: 100.0},
            color: Color::new(1.0, 1.0, 0.0, 1.0),
            direction: 0.05, 
        }

    }

    pub fn update(self:&mut Self, _time_stamp:f64) {
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
}

struct RenderData {
    pub scale_matrix:[f32;16],
    pub mvp_matrix:[f32;16],
    pub color_vec:[f32;4],
    pub program_id: usize, 
}

impl RenderData {
    pub fn new(webgl_renderer:&mut WebGlRenderer) -> Result<Self, &'static str> {
        let program_id:usize = 0;

        /*
        let program_id = webgl_renderer.compile_program(
            include_str!("shaders/simple-vertex.glsl"),
            include_str!("shaders/simple-fragment.glsl")
        )?;
        */

/*
        let buffer_id = webgl_renderer.create_buffer()?;

        let data:Vec<f32> = vec![  
                0.0,1.0, // top-left
                0.0,0.0, //bottom-left
                1.0, 1.0, // top-right
                1.0, 0.0 // bottom-right
        ];
        webgl_renderer.upload_array_buffer(buffer_id, &data, BufferTarget::ArrayBuffer, BufferUsage::StaticDraw)?;

        webgl_renderer.activate_attribute_name_in_current_program("a_vertex", &attributes::AttributeOptions::new(2, DataType::Float))?;

        */
        Ok(Self{
            program_id,
            scale_matrix: [0.0;16],
            mvp_matrix: [0.0;16],
            color_vec: [0.0;4]
        })
    }

    pub fn set_from_state(self:&mut Self, camera_matrix:&[f32;16], state:&State) {
        let mut scratch_matrix:[f32;16] = [0.0;16]; 
        let RenderData {scale_matrix, mvp_matrix, color_vec, ..} = self;
        let State {pos, area, color, ..} = state;


        //scale
        write_scale_matrix(area.width, area.height, 1.0, scale_matrix);
       
        //model-view-projection
        write_position_matrix(pos.x, pos.y, 0.0, &mut scratch_matrix);
        write_multiply_matrix(camera_matrix, &scratch_matrix, mvp_matrix); 

        //color
        color.write_to_v32_4(color_vec);


    }
}

pub fn start(window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {

    let webgl_renderer = start_webgl(window, document, body)?;

    let state = State::new();

    Ok(())
}
