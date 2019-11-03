use awsm_web::errors::Error;
use awsm_web::webgl::{
    AttributeOptions, BufferData, BufferTarget, BufferUsage, DataType, Id, WebGlCommon,
    WebGlRenderer,
};

static QUAD_GEOM_UNIT: [f32; 8] = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];

static BOX_GEOM_UNIT: [f32; 72] = [
    1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, // v0-v1-v2-v3 front
    1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, // v0-v3-v4-v5 right
    1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, // v0-v5-v6-v1 up
    -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, // v1-v6-v7-v2 left
    -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, // v7-v4-v3-v2 down
    1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, // v4-v7-v6-v5 back
];

static BOX_COLORS: [f32; 72] = [
    0.4, 0.4, 1.0, 0.4, 0.4, 1.0, 0.4, 0.4, 1.0, 0.4, 0.4, 1.0, // v0-v1-v2-v3 front(blue)
    0.4, 1.0, 0.4, 0.4, 1.0, 0.4, 0.4, 1.0, 0.4, 0.4, 1.0, 0.4, // v0-v3-v4-v5 right(green)
    1.0, 0.4, 0.4, 1.0, 0.4, 0.4, 1.0, 0.4, 0.4, 1.0, 0.4, 0.4, // v0-v5-v6-v1 up(red)
    1.0, 1.0, 0.4, 1.0, 1.0, 0.4, 1.0, 1.0, 0.4, 1.0, 1.0, 0.4, // v1-v6-v7-v2 left
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, // v7-v4-v3-v2 down
    0.4, 1.0, 1.0, 0.4, 1.0, 1.0, 0.4, 1.0, 1.0, 0.4, 1.0, 1.0, // v4-v7-v6-v5 back
];

static BOX_ELEMENTS: [u8; 36] = [
    0, 1, 2, 0, 2, 3, // front
    4, 5, 6, 4, 6, 7, // right
    8, 9, 10, 8, 10, 11, // up
    12, 13, 14, 12, 14, 15, // left
    16, 17, 18, 16, 18, 19, // down
    20, 21, 22, 20, 22, 23, // back
];

pub static N_BOX_ELEMENTS: u32 = 36;

pub fn create_and_assign_unit_quad_buffer<T: WebGlCommon>(
    webgl_renderer: &mut WebGlRenderer<T>,
) -> Result<Id, Error> {
    let buffer_id = webgl_renderer.create_buffer()?;

    webgl_renderer.upload_buffer_to_attribute(
        buffer_id,
        BufferData::new(
            &QUAD_GEOM_UNIT,
            BufferTarget::ArrayBuffer,
            BufferUsage::StaticDraw,
        ),
        "a_vertex",
        &AttributeOptions::new(2, DataType::Float),
    )?;

    Ok(buffer_id)
}

pub fn create_unit_box_buffers<T: WebGlCommon>(
    webgl_renderer: &mut WebGlRenderer<T>,
) -> Result<(Id, Id, Id), Error> {
    let geom_id = webgl_renderer.create_buffer()?;
    let colors_id = webgl_renderer.create_buffer()?;
    let elements_id = webgl_renderer.create_buffer()?;

    //Gotta use slices since arrays are only impl's up to 32 values
    webgl_renderer.upload_buffer(
        geom_id,
        BufferData::new(
            &BOX_GEOM_UNIT[..],
            BufferTarget::ArrayBuffer,
            BufferUsage::StaticDraw,
        ),
    )?;

    webgl_renderer.upload_buffer(
        colors_id,
        BufferData::new(
            &BOX_COLORS[..],
            BufferTarget::ArrayBuffer,
            BufferUsage::StaticDraw,
        ),
    )?;

    webgl_renderer.upload_buffer(
        elements_id,
        BufferData::new(
            &BOX_ELEMENTS[..],
            BufferTarget::ElementArrayBuffer,
            BufferUsage::StaticDraw,
        ),
    )?;

    Ok((geom_id, colors_id, elements_id))
}
