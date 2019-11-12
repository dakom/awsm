use std::rc::Rc;
use std::cell::RefCell;
use futures::future::{self, FutureExt, TryFutureExt};
use futures::Future;
use awsm_web::webgl::{
    Id,
    WebGl2Renderer,
    ClearBufferMask,
    BufferData,
    BufferTarget,
    BufferUsage
};
use crate::errors::{Error, NativeError};
use crate::gltf::GltfResource;
use gltf::{Document};
use gltf::accessor::{Dimensions, DataType};
use log::info;
use shipyard::*;
use std::borrow::Cow;

pub struct Accessor {
    buffer_id: Id
}

#[derive(Clone)]
pub enum Keyword {
    Loop,
    Continue,
    Break,
    Fn,
    Extern,
}

fn dim_size(type_:Dimensions) -> usize {
    match type_ {
        Dimensions::Scalar => 1,
        Dimensions::Vec2 => 2,
        Dimensions::Vec3 => 3,
        Dimensions::Vec4 | Dimensions::Mat2 => 4,
        Dimensions::Mat3 => 9,
        Dimensions::Mat4 => 16,
    }
}

fn component_size(type_:DataType) -> usize {
    match type_ {
        DataType::I8 | DataType::U8 => 1, //BYTE | UNSIGNED_BYTE
        DataType::I16 | DataType::U16 => 2, //SHORT | UNSIGNED_SHORT
        DataType::U32 | DataType::F32 => 4, //UNSIGNED_INT| FLOAT 
    }
}

/*
fn element_byte_size(accessor:&gltf::accessor::Accessor) -> usize {
    dim_size(accessor.dimensions()) * component_size(accessor.data_type())
}

fn get_byte_length(accessor:&gltf::accessor::Accessor) -> usize {
    accessor.count() * element_byte_size(accessor) 
}

fn get_byte_offset(view:&gltf::buffer::View, accessor:&gltf::accessor::Accessor) -> usize {
    //TODO - followup with https://github.com/gltf-rs/gltf/issues/268
    view.offset() + accessor.offset()
}

fn get_byte_stride_len(view:&gltf::buffer::View, accessor:&gltf::accessor::Accessor) -> usize {
    match view.stride() {
        None => 0,
        Some(stride) => {
            stride * element_byte_size(accessor) 
        }
    }
}

#[derive(Debug)]
struct AccessorInfo {
    base: BaseAccessorInfo,
    sparse: Option<SparseAccessorInfo>
}

#[derive(Debug)]
struct SparseAccessorInfo {
    indices: BaseAccessorInfo,
    values: BaseAccessorInfo
}

#[derive(Debug)]
struct BaseAccessorInfo {
    len: usize,
    offset: usize,
    component_type: DataType,
    dim_type: Dimensions,
    buffer_id: Id,
}

//TODO - implement getting typed data
// https://github.com/dakom/pure3d-typescript/blob/master/src/lib/internal/gltf/gltf-parse/Gltf-Parse-Data-Typed.ts
// https://users.rust-lang.org/t/return-new-vec-or-slice/34542
fn get_accessor_data<'a> (accessor:&gltf::accessor::Accessor, buffers:&'a Vec<Vec<u8>>) -> Cow<'a, [u8]> {

    //TODO - remove the temp Some wrapper
    //https://github.com/gltf-rs/gltf/issues/266
    match Some(accessor.view()) {
        Some(view) => {
            let byte_offset = get_byte_offset(&view, accessor);
            let byte_len = get_byte_length(accessor);
            let byte_end = byte_offset + byte_len;
            let full_buffer_data = &buffers[view.buffer().index()];

            //info!("target length {} start {} end {}", full_buffer_data.len(), byte_offset, byte_end);

            Cow::Borrowed(&full_buffer_data[byte_offset..byte_end])
        },
        None => {
            let n_values = accessor.count() * dim_size(accessor.dimensions());
        }
    }
}

fn make_accessor_info(webgl:&mut WebGl2Renderer, accessor:&gltf::accessor::Accessor, buffer_view_ids:&mut Vec<Id>) -> Result<AccessorInfo, Error> {
    let accessor_id = accessor.index();

    let byte_len = get_byte_length(&accessor);
    //TODO - remove the temp Some wrapper
    //https://github.com/gltf-rs/gltf/issues/266
    match Some(accessor.view()) {
        None => {
            if accessor.sparse().is_none() {
                return Err(NativeError::AccessorSparse.into())
            }

            let buffer_id = webgl.create_buffer()?;
            let data = get_accessor_data(accessor)?;
            //TODO - create and fill buffer with 0's


            Ok(AccessorInfo{
                base: BaseAccessorInfo{
                    len: byte_len,
                    offset: 0,
                    component_type: accessor.data_type(),
                    dim_type: accessor.dimensions(),
                    buffer_id
                },
                sparse: None
             })
        },
        Some(view) => {
            let offset = get_byte_offset(&view, &accessor);
            let stride_len = get_byte_stride_len(&view, &accessor);

            Ok(AccessorInfo{
                base: BaseAccessorInfo {
                    len: byte_len + stride_len,
                    offset,
                    component_type: accessor.data_type(),
                    dim_type: accessor.dimensions(),
                    buffer_id: buffer_view_ids[view.index()]
                },
                sparse: None
            })
        }
    }
}

*/
fn accessor_is_attribute(gltf:&Document, accessor:&gltf::accessor::Accessor) -> bool {
    let accessor_id = accessor.index();

    gltf.nodes().any(|node| {
        if let Some(mesh) = node.mesh() {
            mesh.primitives().any(|primitive| {

                if primitive.indices().map(|acc| acc.index()).contains(&accessor_id) {
                    return true;
                }
                if primitive.attributes().any(|(_, attribute_accessor)| {
                    attribute_accessor.index() == accessor_id
                }) {
                    return true;
                }
                if primitive.morph_targets().any(|morph_target| {
                    morph_target.positions().map(|acc| acc.index()).contains(&accessor_id) 
                        || morph_target.normals().map(|acc| acc.index()).contains(&accessor_id) 
                        || morph_target.tangents().map(|acc| acc.index()).contains(&accessor_id)
                }) {
                    return true;
                }

                false
            })
        } else {
            false
        }
    })
}
pub fn populate_accessors(webgl:&mut WebGl2Renderer, world:&mut World, gltf:&Document, buffer_view_ids:&mut Vec<Id>, buffers:&Vec<Vec<u8>>) -> Result<(), Error> {
    //https://github.com/dakom/pure3d-typescript/blob/master/src/lib/internal/gltf/gltf-parse/Gltf-Parse-Data-Attributes.ts
    //https://github.com/dakom/pure3d-typescript/blob/master/src/lib/internal/gltf/gltf-parse/Gltf-Parse-Data-Info.ts

    for accessor in gltf.accessors() {
        let accessor_id = accessor.index();

        info!("got accessor id {}", accessor_id);

        match accessor.sparse() {
            Some(sparse) => {
                //TODO - handle sparse
                //Maybe GLTF_PARSE_getAccessorTypedData  ?
            },
            None => {
                //TODO - handle non-sparse
            }
        };
    }

    Ok(())
}