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

fn get_byte_length(accessor:&gltf::accessor::Accessor) -> usize {
    accessor.count() * dim_size(accessor.dimensions()) * component_size(accessor.data_type())
}

fn get_byte_offset(view:&gltf::buffer::View, accessor:&gltf::accessor::Accessor) -> usize {
    //TODO - followup with https://github.com/gltf-rs/gltf/issues/268
    view.offset() + accessor.offset()
}

fn get_byte_stride_len(view:&gltf::buffer::View, accessor:&gltf::accessor::Accessor) -> usize {
    match view.stride() {
        None => 0,
        Some(stride) => {
            stride * dim_size(accessor.dimensions()) * component_size(accessor.data_type())
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

fn make_accessor_info(webgl:&mut WebGl2Renderer, accessor:&gltf::accessor::Accessor, buffer_view_ids:&mut Vec<Id>) -> Result<AccessorInfo, Error> {
    let accessor_id = accessor.index();

    let byte_len = get_byte_length(&accessor);
    //TODO - remove the temp Some wrapper
    //https://github.com/gltf-rs/gltf/issues/266

    //TODO - fill out sparse
    match Some(accessor.view()) {
        None => {
            if accessor.sparse().is_none() {
                return Err(NativeError::AccessorSparse.into())
            }

            let buffer_id = webgl.create_buffer()?;
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
pub fn populate_accessors(webgl:&mut WebGl2Renderer, world:&mut World, gltf:&Document, buffer_view_ids:&mut Vec<Id>) -> Result<(), Error> {
    //https://github.com/dakom/pure3d-typescript/blob/master/src/lib/internal/gltf/gltf-parse/Gltf-Parse-Data-Attributes.ts
    //https://github.com/dakom/pure3d-typescript/blob/master/src/lib/internal/gltf/gltf-parse/Gltf-Parse-Data-Info.ts

    for accessor in gltf.accessors() {
        let accessor_id = accessor.index();

        let info = make_accessor_info(webgl, &accessor, buffer_view_ids)?;

        info!("got accessor {:?}", info);

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