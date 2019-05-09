use crate::rust::helpers::data::*;
use crate::rust::helpers::matrix::*;
use awsm_webgl::errors::*;
use awsm_webgl::enums::{BufferTarget, BufferUsage, DataType, PixelFormat};
use awsm_webgl::renderer::WebGlRenderer;
use awsm_webgl::textures::{SimpleTextureOptions, WebGlTextureSource};
use awsm_webgl::*;
use awsm_loaders::*;
use web_sys::{HtmlImageElement};
use futures::future::{Future};

pub struct QuadTextureInstanceData {
    pub pos: Point,
    pub area: Area,
    pub img: HtmlImageElement,
}

impl QuadTextureInstanceData {
    pub fn new() -> impl Future<Item = QuadTextureInstanceData, Error = Error> { 
        image::fetch_image(String::from("http://localhost:31337/sprites/bunnies/bunny.png"))
            .map_err(Error::from)
            .map(|img| {

                let pos = Point{x: 500.0, y: 500.0};
                let area = Area{width: 25.0, height: 32.0};
                let color = Color::new(1.0, 1.0, 0.0, 1.0);

                QuadTextureInstanceData{
                        pos, 
                        area, 
                        img,
                }
            })
    }

    pub fn update(self:&mut Self, _time_stamp:f64) {
    }

}

pub struct QuadTextureRenderData {
    pub scale_matrix:[f32;16],
    pub mvp_matrix:[f32;16],
    pub color_vec:[f32;4], 
    pub program_id: usize,
    pub texture_id: usize,
}

impl QuadTextureRenderData {
    pub fn new(webgl_renderer:&mut WebGlRenderer, instance_data:&QuadTextureInstanceData) -> Result<QuadTextureRenderData, Error> {

        let program_id = webgl_renderer.compile_program(
            include_str!("shaders/Quad-Texture-Vertex.glsl"),
            include_str!("shaders/Quad-Texture-Fragment.glsl")
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

        let texture_id = webgl_renderer.create_texture()?;

        webgl_renderer.assign_simple_texture(
            texture_id, 
            &SimpleTextureOptions{
                pixelFormat: PixelFormat::Rgba,
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::ImageElement(&instance_data.img)
        )?;

        Ok(QuadTextureRenderData{
            scale_matrix: [0.0;16], 
            mvp_matrix: [0.0;16], 
            color_vec: [0.0;4], 
            program_id,
            texture_id
        })
    }

    pub fn update(self:&mut Self, camera_matrix:&[f32;16], instance_data:&QuadTextureInstanceData) {
        let mut scratch_matrix:[f32;16] = [0.0;16]; 
        let QuadTextureRenderData {scale_matrix, mvp_matrix, color_vec, ..} = self;
        let QuadTextureInstanceData {pos, area, ..} = instance_data;

        //scale
        write_scale_matrix(area.width, area.height, 1.0, scale_matrix);
       
        //model-view-projection
        write_position_matrix(pos.x, pos.y, 0.0, &mut scratch_matrix);
        write_multiply_matrix(camera_matrix, &scratch_matrix, mvp_matrix); 

        //color


    }
}