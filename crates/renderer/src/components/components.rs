use shipyard::*;
pub(crate) use crate::primitives::Primitive;


pub fn register_components(world:&mut World) {
    world.register::<Primitive>();
}