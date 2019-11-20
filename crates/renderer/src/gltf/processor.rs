use crate::errors::{Error, NativeError};
use crate::gltf::loader::{GltfResource};
use crate::primitives::*;
use crate::shaders::compile_shader;
use super::accessors::AccessorInfo;
use crate::nodes::*;
use shipyard::*;
use awsm_web::webgl::{ 
    Id, 
    WebGl2Renderer,
    BufferData,
    BufferTarget,
    BufferUsage,
    AttributeOptions,
    VertexArray,
    BeginMode
};
use std::convert::TryInto;

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


        let shader_id = compile_shader(state.webgl)?;

        let vao_id = state.webgl.create_vertex_array()?;
        //Probably some way of making this just one iterator that exists early...
        let mut attributes = Vec::with_capacity(primitive.attributes().len());

        for (semantic, accessor) in primitive.attributes() {
            let buffer_id = upload_accessor(state, &accessor, BufferTarget::ArrayBuffer)?;
            let accessor_info = AccessorInfo::new(&accessor);
            let opts = AttributeOptions::new(accessor_info.data_size, accessor_info.webgl_data_type);
            let attribute_name = match semantic {
                gltf::Semantic::Positions =>  "a_position",
                gltf::Semantic::Normals => "a_normal",
                gltf::Semantic::Tangents => "a_tangent",
                gltf::Semantic::Colors(_color) => "colors",
                gltf::Semantic::TexCoords(_coord) => "texcoords",
                gltf::Semantic::Joints(_joints) => "joints",
                gltf::Semantic::Weights(_weights) => "weights",
                gltf::Semantic::Extras(_extras) => "extras",
            };

            //log::info!("dimensions for {} is {}", attribute_name, accessor_info.dim_size);
            log::info!("attribute {} data buffer id is {:?} for accessor {}, primitive {}, count {}", attribute_name, buffer_id, accessor.index(), primitive.index(), accessor.count());
            if false {
                attributes.push((buffer_id, attribute_name, accessor_info, opts));
            }
        }

        let draw_mode = get_primitive_mode(&primitive);
        let (elements_id, draw_info) = match primitive.indices() {
            Some(accessor) => {
                let accessor_info = AccessorInfo::new(&accessor);
                let buffer_id = upload_accessor(state, &accessor, BufferTarget::ElementArrayBuffer)?;
                log::info!("elements data buffer id is {:?} for accessor {}, primitive {}, count {}", buffer_id, accessor.index(), primitive.index(), accessor.count());
                (Some(buffer_id), PrimitiveDraw::Elements(draw_mode, accessor.count().try_into().unwrap(), accessor_info.webgl_data_type, accessor.offset().try_into().unwrap()))
            },

            //TODO
            None => (None, PrimitiveDraw::Direct(draw_mode, 36, 0))
        };


        /*
            Ideas: 
            1. We have info on the semantics in attributes - could use that here...
            2. Maybe the attributes for this primitive should be changeable at runtime - so these could be a component?
            3. Probably better to start with hardcoded / inline here and then work backwards
        */


        state.webgl.assign_vertex_array( 
            vao_id, 
            elements_id, 
            &attributes.iter().map(|(buffer_id, attribute_name, _dim_size, opts)| {
                VertexArray{
                    attribute_name,
                    buffer_id: *buffer_id,
                    opts: &opts
                }
            }).collect::<Vec<VertexArray>>()
        )?;

        add_node(state.world, NodeData::Primitive(Primitive{shader_id, vao_id, draw_info, }), None, None, None, None);
    }

    Ok(())
}

fn get_primitive_mode(primitive:&gltf::mesh::Primitive) -> BeginMode {
    match primitive.mode() {
        gltf::mesh::Mode::Points => BeginMode::Points,
        gltf::mesh::Mode::Lines => BeginMode::Lines,
        gltf::mesh::Mode::LineLoop => BeginMode::LineLoop,
        gltf::mesh::Mode::LineStrip => BeginMode::LineStrip,
        gltf::mesh::Mode::Triangles => BeginMode::Triangles,
        gltf::mesh::Mode::TriangleStrip => BeginMode::TriangleStrip,
        gltf::mesh::Mode::TriangleFan => BeginMode::TriangleFan,
    }
}

//If the view is non-sparse, upload as-is
//Otherwise, create new data and upload
//Either way, return the buffer id
fn upload_accessor(state:&mut ProcessState, accessor:&gltf::accessor::Accessor, target:BufferTarget) -> Result<Id, Error> {

    match accessor.sparse() {
        Some(_sparse) => {
            match accessor.view() {
                Some(_view) => {
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
            let view = accessor.view().ok_or(Error::from(NativeError::AccessorView))?;
            upload_buffer_view(state, &view, target)
        }
    }
}

//Upload the buffer view if and only if there isn't already an id for that specific view
//In either case, return the Id
fn upload_buffer_view(state:&mut ProcessState, view:&gltf::buffer::View, target:BufferTarget) -> Result<Id, Error> {

    let ProcessState {webgl, resource, buffer_view_ids, ..} = state;
    let GltfResource {buffers, ..} = resource; 

    let buffer_view_id = view.index();

    if buffer_view_ids[buffer_view_id].is_none() {

        let buffer_id = webgl.create_buffer()?;
        let data = BufferData::new(
            super::buffer_view::get_buffer_view_data(&view, &buffers),
            target,
            BufferUsage::StaticDraw
        );

        //log::info!("len {}: {:?}", data.values.len(), &data.values);

        webgl.upload_buffer(buffer_id, data)?;
        buffer_view_ids[buffer_view_id] = Some(buffer_id);
    }

    Ok(buffer_view_ids[buffer_view_id].unwrap())
}

