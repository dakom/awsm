use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use super::events::*;
use wasm_bindgen_futures::futures_0_3::future_to_promise;
use web_sys::{HtmlCanvasElement};
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use shared::state::*;
use awsm_web::loaders::fetch;
use awsm_web::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
    ClearBufferMask,
    WebGlCommon,
    WebGl1Renderer,
    Id,
    TextureTarget,
    SimpleTextureOptions,
    PixelFormat,
    WebGlTextureSource,
    BufferData,
    BufferTarget,
    BufferUsage,
    DataType,
    AttributeOptions,
    VertexArray
};
use super::renderer::Renderer;

pub fn load_assets(renderer:Rc<RefCell<Renderer>>) {

    future_to_promise({
            async move {
                //load everything - then we can borrow renderer mutably since it'll be sync
                let vertex = fetch::text("media/shaders/vertex.glsl").await?;
                let fragment = fetch::text("media/shaders/fragment.glsl").await?;
                let image = fetch::image("media/images/smiley.svg").await?;


                let mut renderer= renderer.borrow_mut();

                //PROGRAM
                let program_id = renderer.webgl.compile_program(&vertex, &fragment)?;
                renderer.program_id = Some(program_id);

                //TEXTURE
                let texture_id = renderer.webgl.create_texture()?;
                renderer.webgl.assign_simple_texture(
                        texture_id,
                        TextureTarget::Texture2d,
                        &SimpleTextureOptions {
                            pixel_format: PixelFormat::Rgba,
                            ..SimpleTextureOptions::default()
                        },
                        &WebGlTextureSource::ImageElement(&image),
                    )?;

                renderer.texture_id = Some(texture_id);

                //QUAD GEOM DATA 
                let buffer_id = renderer.webgl.create_buffer()?;

                renderer.webgl.upload_buffer(
                    buffer_id,
                    BufferData::new(
                        &QUAD_GEOM_UNIT,
                        BufferTarget::ArrayBuffer,
                        BufferUsage::StaticDraw,
                    ),
                )?;

                //VAO
                let vao_id = renderer.webgl.create_vertex_array()?;
                renderer.webgl.assign_vertex_array(
                        vao_id,
                        None,
                        &vec![VertexArray {
                            attribute_name: "a_vertex",
                            buffer_id,
                            opts: &AttributeOptions::new(2, DataType::Float),
                        }],
                    )?;
                renderer.vao_id = Some(vao_id);

                //DONE - tell core :)
                //renderer.send_event(&Event::Loaded);

                Ok(JsValue::null())
            }
        });
}

static QUAD_GEOM_UNIT: [f32; 8] = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];