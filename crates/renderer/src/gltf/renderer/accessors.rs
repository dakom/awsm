use std::rc::Rc;
use std::cell::RefCell;
use futures::future::{self, FutureExt, TryFutureExt};
use futures::Future;
use awsm_web::webgl::{
    WebGl2Renderer,
    ClearBufferMask,
    BufferData,
    BufferTarget,
    BufferUsage
};
use awsm_web::errors::Error;
use crate::gltf::GltfResource;
use gltf::{Document};
use log::info;
use shipyard::*;

pub fn upload_accessors(webgl:&mut WebGl2Renderer, gltf:&Document, buffers:&Vec<Vec<u8>>) -> Result<(), Error> {
    //the spec says that if an accessor index is not found in primitive.indices then it is ARRAY_BUFFER
    //otherwise it is ELEMENT_ARRAY_BUFFER
    //conceptually the spec also dictates that this should match bufferView.target but this is more reliable
    //See: https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#primitiveindices
    let primitive_indices_list:Vec<usize> = 
        gltf
            .meshes()
            .flat_map(|mesh| mesh.primitives())
            .flat_map(|primitive| primitive.indices())
            .map(|accessor| accessor.index())
            .collect();

    //https://github.com/dakom/pure3d-typescript/blob/master/src/lib/internal/gltf/gltf-parse/Gltf-Parse-Data-Attributes.ts
    for (i, accessor) in gltf.accessors().enumerate() {
        let buffer_id = webgl.create_buffer()?;
        let accessor_id = accessor.index();

        let is_elements = primitive_indices_list.contains(&accessor_id);
        match accessor.sparse() {
            Some(sparse) => {
                //TODO - handle sparse
            },
            None => {
                let buffer_data = super::buffer_view::get_buffer_view_data(&accessor.view(), buffers);

                //TODO - handle non-sparse
                /*
                    const bufferView = gltf.bufferViews[info.bufferViewIndex];
                    const byteOffset = bufferView.byteOffset ? bufferView.byteOffset : 0;

                    bufferViewInfo.set(info.bufferViewIndex, 
                        {
                            rendererBufferId: Symbol(`${info.bufferViewIndex}`),
                            buffer: buffers[info.bufferIndex].slice(byteOffset, byteOffset + bufferView.byteLength)
                        }
                    )

                const bvInfo = bufferViewInfo.get(info.bufferViewIndex);
                rendererBufferId = bvInfo.rendererBufferId;
                buffer = bvInfo.buffer;
                */
            }
        };

        
        //let data = BufferData::new(data, BufferTarget::
        //let data_buffer = &buffers[i];
        //todo - maybe should be per-accessor?
        info!("uploading accessor index {} ({}) to buffer id {:?}... is elements: {}", i, accessor.index(), buffer_id, is_elements);
    }

    Ok(())
}