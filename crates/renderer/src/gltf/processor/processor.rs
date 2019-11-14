use crate::errors::{Error, NativeError};
use crate::gltf::loader::{GltfResource};
use shipyard::*;
use awsm_web::webgl::{ 
    Id, 
    WebGl2Renderer,
    BufferData,
    BufferTarget,
    BufferUsage,
    VertexArray,
    AttributeOptions,
    DataType
};

pub struct ProcessState <'a> {
    pub resource:&'a GltfResource,
    pub world:&'a mut World,
    pub webgl:&'a mut WebGl2Renderer,

    //Just a local holder to help de-dup data
    buffer_view_ids:Vec<Option<Id>>,
}

impl <'a> ProcessState<'a> {
    pub fn new(resource:&'a GltfResource, world:&'a mut World, webgl:&'a mut WebGl2Renderer) -> Self {
        let buffer_view_ids:Vec<Option<Id>> = vec![None;resource.gltf.views().len()];

        Self{
            resource,
            world,
            webgl,
            buffer_view_ids
        }
    }
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

        let vao_id = state.webgl.create_vertex_array()?;
        let elements_id = match primitive.indices() {
            Some(accessor) => {
                let buffer_id = upload_accessor(state, &accessor, BufferTarget::ElementArrayBuffer)?;
                log::info!("elements data buffer id is {:?} for accessor {}, primitive {}", buffer_id, accessor.index(), primitive.index());
                Some(buffer_id)
            },
            None => None
        };

        //Probably some way of making this just one iterator that exists early...
        let mut attributes = Vec::with_capacity(primitive.attributes().len());

        for (semantic, accessor) in primitive.attributes() {
            let buffer_id = upload_accessor(state, &accessor, BufferTarget::ArrayBuffer)?;
            let data_type = super::accessors::get_accessor_webgl_data_type(accessor.data_type());
            let data_size = super::accessors::get_accessor_data_size(accessor.data_type());
            let opts = AttributeOptions::new(data_size, data_type);
            let attribute_name = match semantic {
                gltf::Semantic::Positions => "a_Position",
                gltf::Semantic::Normals => "a_Normal",
                gltf::Semantic::Tangents => "a_Tangent",
                gltf::Semantic::Colors(color) => "colors",
                gltf::Semantic::TexCoords(coord) => "texcoords",
                gltf::Semantic::Joints(joints) => "joints",
                gltf::Semantic::Weights(weights) => "weights",
                gltf::Semantic::Extras(extras) => "extras",
            };

            attributes.push((buffer_id, attribute_name, opts));
            log::info!("attribute {} data buffer id is {:?} for accessor {}, primitive {}", attribute_name, buffer_id, accessor.index(), primitive.index());
        }

        /* TODO - compile shader!
            Ideas: 
            1. We have info on the semantics in attributes - could use that here...
            2. Maybe the attributes for this primitive should be changeable at runtime - so these could be a component?
            3. Probably better to start with hardcoded / inline here and then work backwards
        */

        /*
        This is actually fine, probably, just need shader enabled first (hopefully)
        state.webgl.assign_vertex_array( 
            vao_id, 
            elements_id, 
            &attributes.iter().map(|(buffer_id, attribute_name, opts)| {
                VertexArray{
                    attribute_name,
                    buffer_id: *buffer_id,
                    opts: &opts
                }
            }).collect::<Vec<VertexArray>>()
        )?;
        */
    }

    Ok(())
}


//If the view is non-sparse, upload as-is
//Otherwise, create new data and upload
//Either way, return the buffer id
fn upload_accessor(state:&mut ProcessState, accessor:&gltf::accessor::Accessor, target:BufferTarget) -> Result<Id, Error> {

    match accessor.sparse() {
        Some(sparse) => {
            //Some wrapper is temporary - see https://github.com/gltf-rs/gltf/issues/266
            match Some(accessor.view()) {
                Some(view) => {
                    //TODO
                    log::info!("get the typed data from buffer view");
                },
                None => {
                    //TODO
                    log::info!("create empty (filled with 0's) typed data from buffer view");
                }
            }

            //TODO
            log::info!("replace typed data with sparse info");

            Err(NativeError::Wip.into())
        },
        None => {
            //Some wrapper is temporary - see https://github.com/gltf-rs/gltf/issues/266
            let view = Some(accessor.view()).ok_or(Error::from(NativeError::AccessorView))?;
            upload_buffer_view(state, &view, target)
        }
    }
}

//Upload the buffer view if and only if there isn't already an id for that specific view
//In either case, return the Id
fn upload_buffer_view(state:&mut ProcessState, view:&gltf::buffer::View, target:BufferTarget) -> Result<Id, Error> {

    let ProcessState {webgl, world, resource, buffer_view_ids} = state;
    let GltfResource {gltf, buffers, images} = resource; 

    let buffer_view_id = view.index();

    if buffer_view_ids[buffer_view_id].is_none() {

        let buffer_id = webgl.create_buffer()?;
        let data = BufferData::new(
            super::buffer_view::get_buffer_view_data(&view, &buffers),
            BufferTarget::ArrayBuffer,
            BufferUsage::StaticDraw
        );

        webgl.upload_buffer(buffer_id, data)?;
        buffer_view_ids[buffer_view_id] = Some(buffer_id);
    }

    Ok(buffer_view_ids[buffer_view_id].unwrap())
}

