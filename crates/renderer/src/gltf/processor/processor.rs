use crate::errors::{Error, NativeError};
use crate::gltf::loader::{GltfResource};
use shipyard::*;
use awsm_web::webgl::{ Id, WebGl2Renderer};

pub struct ProcessState <'a, 'b, 'c> {
    pub resource:&'a GltfResource,
    pub world:&'b World,
    pub webgl:&'c WebGl2Renderer,
}

pub fn process_scene(state:ProcessState, scene:&gltf::scene::Scene) -> Result<(), Error> {
    let mut state = state;

    fn traverse_node_root(state:&mut ProcessState, node:&gltf::Node) -> Result<(), Error> 
    {
        log::info!("processing node {}", node.index());
        if let Some(mesh) = node.mesh() {
            process_mesh(state, &mesh)?;
        }
        for node in node.children() {
            traverse_node_root(state, &node)?;
        } 
        Ok(())
    };

    for node in scene.nodes() {
        traverse_node_root(&mut state, &node)?;
    } 
    Ok(())
}

pub fn process_mesh(state:&mut ProcessState, mesh:&gltf::mesh::Mesh) -> Result<(), Error> {

    for primitive in mesh.primitives() {
        let primitive_id = primitive.index();

        for (semantic, accessor) in primitive.attributes() {
            let accessor_id = accessor.index();

            match accessor.sparse() {
                Some(sparse) => {
                    match Some(accessor.view()) {
                        Some(view) => {
                            //TODO - get the typed data from buffer view
                            log::info!("get the typed data from buffer view");
                        },
                        None => {
                            //TODO - create empty (filled with 0's) typed data from buffer view
                            log::info!("create empty (filled with 0's) typed data from buffer view");
                        }
                    }

                    //TODO - replace typed data with sparse info
                    log::info!("replace typed data with sparse info");
                },
                None => {
                    //TODO - create a new buffer_id for the buffer_view if and only if it hasn't already been done
                    log::info!("create a new buffer_id for the buffer_view if and only if it hasn't already been done");
                }
            };
            log::info!("processing accessor {} for primitive {}", accessor_id, primitive_id);
        }
    }

    Ok(())
}