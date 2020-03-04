use awsm_web::webgl::{Id, DataType, BeginMode};
use shipyard::prelude::*;

pub struct Primitive {
    pub shader_id: Id,
    pub vao_id: Id,
    pub draw_info:PrimitiveDraw
} 

pub enum PrimitiveDraw {
    //count, DataType, offset
    Elements(BeginMode, u32, DataType, u32),
    //TODO - update as needed... just copied elements for now
    Direct(BeginMode, u32, u32)
}

