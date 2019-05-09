use crate::rust::helpers::data::*;
use crate::rust::helpers::matrix::*;
use awsm_webgl::errors::*;
use awsm_webgl::enums::{BufferTarget, BufferUsage, DataType, PixelFormat};
use awsm_webgl::renderer::WebGlRenderer;
use awsm_webgl::textures::{SimpleTextureOptions, WebGlTextureSource};
use awsm_webgl::*;
use super::instancing_data::*;

pub struct InstancingRenderData {
    pub scale_matrix:[f32;16],
    pub mvp_matrix:[f32;16],
    pub program_id: usize,
    pub texture_id: usize,
}

impl InstancingRenderData {
    pub fn new(webgl_renderer:&mut WebGlRenderer, instance_data:&InstancingInstanceData) -> Result<InstancingRenderData, Error> {

        webgl_renderer.create_extension_instanced_arrays()?;

        let program_id = webgl_renderer.compile_program(
            include_str!("shaders/Instancing-Vertex.glsl"),
            include_str!("shaders/Instancing-Fragment.glsl")
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

        //scale is constant for all bunnies
        let mut scale_matrix = [0.0;16];
        write_scale_matrix(instance_data.area.width, instance_data.area.height, 1.0, &mut scale_matrix);

        Ok(InstancingRenderData{
            scale_matrix, 
            mvp_matrix: [0.0;16], 
            program_id,
            texture_id
        })
    }

    pub fn update(self:&mut Self, camera_matrix:&[f32;16], _area:&Area, pos:&Point) {
        let mut scratch_matrix:[f32;16] = [0.0;16]; 
        let InstancingRenderData {mvp_matrix, ..} = self;

        //model-view-projection
        write_position_matrix(pos.x, pos.y, 0.0, &mut scratch_matrix);
        write_multiply_matrix(camera_matrix, &scratch_matrix, mvp_matrix); 

    }
}

