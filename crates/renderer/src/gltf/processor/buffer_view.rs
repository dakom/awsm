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

    //info!("target length {} start {} end {}", full_buffer_data.len(), byte_offset, byte_end);

    &full_buffer_data[byte_offset..byte_end]
}