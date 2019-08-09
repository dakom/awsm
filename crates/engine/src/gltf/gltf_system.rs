use crate::errors::{Error, NativeError};
use crate::scene::{Parent};
use specs::prelude::*;
use super::{load_gltf, GltfResource};

/// root nodes can be derived from the gltf scene
/// or a direct list of indices
pub enum GltfSceneRoot <'a> {
    Index(usize),
    NodeList(&'a [usize])
}


/// will create the entities and their Parent+GltfNode components according to the gltf doc
pub(crate) fn setup_gltf( world:&mut World, gltf_resource:GltfResource, scene_root:GltfSceneRoot) -> Result<(), Error> {
    world.register::<GltfNode>();

    create_hierarchy_from_gltf(world, gltf_resource.gltf, scene_root)

}

fn create_hierarchy_from_gltf(world:&mut World, gltf:gltf::Document, scene_root:GltfSceneRoot) -> Result<(), Error> {

    let root_nodes = match scene_root {
        GltfSceneRoot::NodeList(list) => {
            let mut gltf_nodes = Vec::with_capacity(list.len());

            for index in list.iter() {
                let index = *index;
                match gltf.nodes().nth(index) {
                    Some(node) => gltf_nodes.push(node),
                    None => return Err(NativeError::NodeMissing(index).into()) 
                }
            }

            Ok(gltf_nodes)
        },
        GltfSceneRoot::Index(scene_index) => {
                gltf
                    .scenes()
                    .nth(scene_index)
                    .map(|scene| scene.nodes().collect::<Vec<gltf::Node>>())
                    .ok_or(NativeError::SceneMissing)
        }
    }?;

    _create_hierarchy_from_gltf(root_nodes, None, world)
}


fn _create_hierarchy_from_gltf<'a, I>(level_nodes:I, parent: Option<(&gltf::Node, Entity)>, world:&mut World) -> Result<(), Error>
    where
        I: IntoIterator<Item = gltf::Node<'a>>
{ 


    //let parent_index = parent_node.map(|n| n.index());

    for node in level_nodes.into_iter() {
        let entity = world.create_entity().build();
        
        world
            .write_storage::<GltfNode>()
            .insert(entity, GltfNode { index: node.index() })?;

        if let Some((parent_node, parent_entity)) = parent {
            world
                .write_storage::<Parent>()
                .insert(entity, Parent {entity: parent_entity})?;
        }
       
        let children = node.children();
        if children.len() > 0 {
            _create_hierarchy_from_gltf(children, Some((&node, entity)), world)?;
        }             
    }

    Ok(())
}

/// glTF hierarchy components
pub struct GltfNode {
    pub index: usize,
}

impl Component for GltfNode {
    type Storage = VecStorage<Self>;
}
