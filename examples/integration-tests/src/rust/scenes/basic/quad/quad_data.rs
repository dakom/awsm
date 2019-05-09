use crate::rust::helpers::data::*;
use crate::rust::helpers::matrix::*;
use awsm_webgl::errors::*;
use awsm_webgl::enums::{BufferTarget, BufferUsage, DataType};
use awsm_webgl::renderer::WebGlRenderer;
use awsm_webgl::*;

pub struct QuadInstanceData {
    pub pos: Point,
    pub area: Area,
    pub color: Color,
    pub direction: f64,
}

impl QuadInstanceData {
    pub fn new() -> QuadInstanceData { 

        let pos = Point{x: 500.0, y: 500.0};
        let area = Area{width: 300.0, height: 100.0};
        let color = Color::new(1.0, 1.0, 0.0, 1.0);

        QuadInstanceData{
                pos, 
                area, 
                color, 
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

pub struct QuadRenderData {
    pub scale_matrix:[f32;16],
    pub mvp_matrix:[f32;16],
    pub color_vec:[f32;4],
    pub program_id: usize, 
}

impl QuadRenderData {
    pub fn new(webgl_renderer:&mut WebGlRenderer) -> Result<QuadRenderData, Error> {
        let program_id = webgl_renderer.compile_program(
            include_str!("shaders/Quad-Vertex.glsl"),
            include_str!("shaders/Quad-Fragment.glsl")
        )?;

        let buffer_id = webgl_renderer.create_buffer()?;

        let data:Vec<f32> = vec![  
                0.0,1.0, // top-left
                0.0,0.0, //bottom-left
                1.0, 1.0, // top-right
                1.0, 0.0 // bottom-right
        ];
        webgl_renderer.upload_array_buffer(buffer_id, &data, BufferTarget::ArrayBuffer, BufferUsage::StaticDraw)?;

        webgl_renderer.activate_attribute_name_in_current_program("a_vertex", &attributes::AttributeOptions::new(2, DataType::Float))?;

        Ok(QuadRenderData{
            program_id,
            scale_matrix: [0.0;16],
            mvp_matrix: [0.0;16],
            color_vec: [0.0;4]
        })
    }

    pub fn update(self:&mut Self, camera_matrix:&[f32;16], instance_data:&QuadInstanceData) {
        let mut scratch_matrix:[f32;16] = [0.0;16]; 
        let QuadRenderData {scale_matrix, mvp_matrix, color_vec, ..} = self;
        let QuadInstanceData {pos, area, color, ..} = instance_data;

        //scale
        write_scale_matrix(area.width, area.height, 1.0, scale_matrix);
       
        //model-view-projection
        write_position_matrix(pos.x, pos.y, 0.0, &mut scratch_matrix);
        write_multiply_matrix(camera_matrix, &scratch_matrix, mvp_matrix); 

        //color
        color.write_to_v32_4(color_vec);


    }
}