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
    pub program_id: usize,
    pub texture_id: usize,
    pub pos_buffer_id: usize,
}

impl InstancingRenderData {
    pub fn new(webgl_renderer:&mut WebGlRenderer, instance_data:&InstancingInstanceData) -> Result<InstancingRenderData, Error> {

        webgl_renderer.create_extension_instanced_arrays()?;

        let program_id = webgl_renderer.compile_program(
            include_str!("shaders/Instancing-Vertex.glsl"),
            include_str!("shaders/Instancing-Fragment.glsl")
        )?;

        let quad_buffer_id = webgl_renderer.create_buffer()?;
        let data:Vec<f32> = vec![  
                0.0,1.0, // top-left
                0.0,0.0, //bottom-left
                1.0, 1.0, // top-right
                1.0, 0.0 // bottom-right
        ];
        webgl_renderer.upload_array_buffer(quad_buffer_id, &data, BufferTarget::ArrayBuffer, BufferUsage::StaticDraw)?;
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

        //create a new buffer for uploading instancing data
        let pos_buffer_id = webgl_renderer.create_buffer()?;
        Ok(InstancingRenderData{
            scale_matrix, 
            program_id,
            texture_id,
            pos_buffer_id
        })
    }

}

