use gltf::buffer::View;
use std::rc::Rc;
use std::cell::RefCell;
use futures::future::{self, FutureExt, TryFutureExt};
use futures::Future;
use awsm_web::webgl::{
    WebGl2Renderer,
    ClearBufferMask,
    BufferData,
    BufferTarget,
    BufferUsage,
    Id
};
use awsm_web::errors::Error;
use crate::gltf::GltfResource;
use gltf::{Document};
use log::info;
use shipyard::*;

pub fn get_buffer_view_data <'a>(view:&View, buffers:&'a Vec<Vec<u8>>) -> &'a [u8] {
    let byte_offset = view.offset();
    let byte_length = view.length();
    let byte_end = byte_offset + byte_length;
    let full_buffer_data = &buffers[view.buffer().index()];

    info!("target length {} start {} end {}", full_buffer_data.len(), byte_offset, byte_end);

    &full_buffer_data[byte_offset..byte_end]
}

pub fn upload_buffer_views(webgl:&mut WebGl2Renderer, gltf:&Document, buffers:&Vec<Vec<u8>>) -> Result<Vec<Id>, Error> {
    //the spec says that if an accessor index is not found in primitive.indices then it is ARRAY_BUFFER
    //otherwise it is ELEMENT_ARRAY_BUFFER
    //conceptually the spec also dictates that this should match bufferView.target but setting target is not actually required 
    //See: https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#primitiveindices
    let primitive_indices_list:Vec<usize> = 
        gltf
            .meshes()
            .flat_map(|mesh| mesh.primitives())
            .flat_map(|primitive| primitive.indices())
            .map(|accessor| accessor.index())
            .collect();

    let get_target = |view_id:usize| {
        if gltf.accessors()
            .filter(|accessor| accessor.view().index() == view_id)
            .find(|accessor| {
                let accessor_id = accessor.index();
                primitive_indices_list.contains(&accessor_id)
            })
            .is_some() 
        {
            BufferTarget::ElementArrayBuffer
        } else {
            BufferTarget::ArrayBuffer
        }
    };

    gltf
        .views()
        .map(|view| {
            //See: https://github.com/dakom/pure3d-typescript/blob/master/src/lib/internal/gltf/gltf-parse/Gltf-Parse-Data-Attributes.ts
            let buffer_id = webgl.create_buffer()?;
            let data = BufferData::new(
                get_buffer_view_data(&view, buffers),
                get_target(view.index()),
                BufferUsage::StaticDraw
            );

            info!("uploaded buffer... target {}", if data.target == BufferTarget::ElementArrayBuffer { "elements" } else { "array" });

            webgl.upload_buffer(buffer_id, data)?;


            Ok(buffer_id)
        })
        .collect()
}