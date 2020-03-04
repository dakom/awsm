use shipyard::prelude::*;
pub use crate::primitives::Primitive;
pub use crate::transform::*;
pub use crate::camera::*;
pub use crate::nodes::{Node};

pub fn register_components(world:&mut World) {
    world.register::<Node>();
    world.register::<Primitive>();
    world.register::<CameraView>();
    world.register::<CameraProjection>();
    world.register::<Translation>();
    world.register::<Rotation>();
    world.register::<Scale>();
    world.register::<LocalTransform>();
    world.register::<WorldTransform>();
}