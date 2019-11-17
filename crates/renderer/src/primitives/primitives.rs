use awsm_web::webgl::{Id, DataType, BeginMode};
use shipyard::*;

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


pub fn create_primitive(world:&mut World, primitive:Primitive) {
    world.run::<(EntitiesMut, &mut Primitive), _>(|(mut entities, mut primitives)| {
        entities.add_entity(&mut primitives, primitive);
    });
}
