use shipyard::*;
pub(crate) use crate::primitives::Primitive;
pub(crate) use crate::camera::Camera;


pub fn register_components(world:&mut World) {
    world.register::<Primitive>();
    world.register::<Camera>();
}