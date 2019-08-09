mod camera;
mod id;
mod light;
mod material;
mod mesh;
mod skin;
mod transform;
mod hierarchy;

use self::camera::*;
use self::id::*;
use self::light::*;
use self::material::*;
use self::mesh::*;
use self::skin::*;
use self::transform::*;
pub use self::hierarchy::*;

use specs::{World, WorldExt};
use crate::errors::{Error};

pub fn setup_scene(world:&mut World) -> Result<(), Error> {
    let hierarchy = setup_hierarchy(world);
    Ok(())
}
