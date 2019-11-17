use gltf::accessor::{DataType, Dimensions};

pub struct AccessorInfo {
    pub dim_size:usize,
    pub data_size:u8,
    pub webgl_data_type:awsm_web::webgl::DataType
}

impl AccessorInfo {
    pub fn new(accessor:&gltf::accessor::Accessor) -> Self {
        Self{
            dim_size: get_accessor_dim_size(accessor.dimensions()),
            data_size: get_accessor_data_size(accessor.data_type()),
            webgl_data_type: get_accessor_webgl_data_type(accessor.data_type()),
        }
    }
}
            
fn get_accessor_dim_size(type_:gltf::accessor::Dimensions) -> usize {
    match type_ {
        Dimensions::Scalar => 1,
        Dimensions::Vec2 => 2,
        Dimensions::Vec3 => 3,
        Dimensions::Vec4 | Dimensions::Mat2 => 4,
        Dimensions::Mat3 => 9,
        Dimensions::Mat4 => 16,
    }
}

fn get_accessor_data_size(type_:DataType) -> u8 {
    match type_ {
        DataType::I8 | DataType::U8 => 1, //BYTE | UNSIGNED_BYTE
        DataType::I16 | DataType::U16 => 2, //SHORT | UNSIGNED_SHORT
        DataType::U32 | DataType::F32 => 4, //UNSIGNED_INT| FLOAT 
    }
}

fn get_accessor_webgl_data_type(gltf_type:DataType) -> awsm_web::webgl::DataType {

    match gltf_type {
        DataType::I8 => awsm_web::webgl::DataType::Byte,
        DataType::U8 => awsm_web::webgl::DataType::UnsignedByte,
        DataType::I16 => awsm_web::webgl::DataType::Short, 
        DataType::U16 => awsm_web::webgl::DataType::UnsignedShort, 
        DataType::U32 => awsm_web::webgl::DataType::UnsignedInt, 
        DataType::F32 => awsm_web::webgl::DataType::Float, 
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