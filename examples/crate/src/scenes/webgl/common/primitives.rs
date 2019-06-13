use awsm::webgl::{Id, WebGlRenderer, BufferTarget, BufferUsage, AttributeOptions, DataType};
use awsm::errors::{Error};

pub fn create_unit_quad_buffer(webgl_renderer:&mut WebGlRenderer) -> Result<Id, Error> {
    let buffer_id = webgl_renderer.create_buffer()?;

    webgl_renderer.upload_buffer_to_attribute_name(
        buffer_id,
        &vec![  
            0.0,1.0, // top-left
            0.0,0.0, //bottom-left
            1.0, 1.0, // top-right
            1.0, 0.0 // bottom-right
        ],
        BufferTarget::ArrayBuffer,
        BufferUsage::StaticDraw,
        "a_vertex",
        &AttributeOptions::new(2, DataType::Float)
    )?;

    Ok(buffer_id)
}
