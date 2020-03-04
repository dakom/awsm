use awsm_web::webgl::{WebGl2Renderer, Id};
use crate::errors::{Error};

pub struct ShaderSettings{
    pub has_position: bool
}

impl ShaderSettings {
    pub fn get_hash(&self) -> u32 {
        //TODO - implement hasher or derive
        42
    }
}

const PRIMITIVE_VERT:&str = include_str!("glsl/primitive.vert");

const MATERIAL_FRAG:&str = include_str!("glsl/material.frag");

pub fn compile_shader(webgl:&mut WebGl2Renderer) -> Result<Id, Error> {

    let shader_settings = ShaderSettings {
        has_position: true
    };
    //TODO - look it up in hash map and return early if found 
    let _shader_hash = shader_settings.get_hash();

    let vertex_shader = PRIMITIVE_VERT;
    let fragment_shader = MATERIAL_FRAG;
    let program_id = webgl.compile_program(&vertex_shader, &fragment_shader)?;

    Ok(program_id)
}