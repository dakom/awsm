use specs::{World, WorldExt};
use specs::prelude::*;
use specs_hierarchy::{Hierarchy, HierarchySystem};
use gltf::{Document};
use crate::errors::{Error, NativeError};
use crate::gltf::{GltfNode};
use log::{info};

//Main hierarchy components
pub struct Parent {
    pub entity: Entity,
}

impl Component for Parent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl specs_hierarchy::Parent for Parent {
    fn parent_entity(&self) -> Entity {
        self.entity
    }
}


/// setup the hierarchy system - called once per app
pub fn setup_hierarchy(world:&mut World) -> HierarchySystem<Parent> {
    let mut system = HierarchySystem::<Parent>::new(world);
    System::setup(&mut system, world);
    system
}




pub fn log_hierarchy(world:&World) {
        let parents = world.read_storage::<Parent>();
        let gltf_nodes = world.read_storage::<GltfNode>();

        for (entity, _) in (&world.entities(), !&parents).join() {
            let gltf_node = gltf_nodes.get(entity).unwrap();
            info!("{:?} is a root node", gltf_node.index)
        }

        for (entity, parent) in (&world.entities(), &parents).join() { 
            let gltf_node = gltf_nodes.get(entity).unwrap();
            let gltf_parent_node = gltf_nodes.get(parent.entity).unwrap();
            info!("{:?} is a child node of {:?}", gltf_node.index, gltf_parent_node.index)
            //let gltf_node_parent = gltf_nodes.get(parent.entity).unwrap();
            //info!("{:?} is the child of {:?} ", gltf_node_parent.index)
        }
}
