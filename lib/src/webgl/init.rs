use crate::errors::{Error};
use crate::helpers::{clone_to_vec_f32};
use std::collections::HashMap;
use std::cell::Cell;
use beach_map::{BeachMap, DefaultVersion};
use wasm_bindgen::prelude::{JsValue};
use web_sys::{WebGlVertexArrayObject, HtmlCanvasElement, WebGlTexture, WebGlBuffer};
use super::{Id, TextureSamplerInfo, BufferTarget, ProgramInfo, WebGlContext, WebGlContextOptions, GlQuery, get_webgl_context};
use super::funcs::{FuncSettings};
use super::toggles::{ToggleFlags};

/*
 * extension_lookup, attribute_lookup, and uniform_lookup are hashmaps
 * however, they are only populated at junctures where computation is already expensive
 *
 * everything else is a more efficient data structure
 * right now it's somewhat arbitrary that buffer/attribute/uniform setting happens through an
 * immutable api while textures and flags and things are though mutable.
 *
 * however that might come in handy down the line
 */

pub struct WebGlRenderer {
    pub gl:WebGlContext,
    pub canvas: HtmlCanvasElement,
   
    //really just local to the module
    pub(super) last_width: u32,
    pub(super) last_height: u32,

    pub(super) current_program_id: Option<Id>,
    pub(super) program_lookup: BeachMap<DefaultVersion, ProgramInfo>, 

    pub(super) current_buffer_id: Cell<Option<Id>>,
    pub(super) current_buffer_target: Cell<Option<BufferTarget>>,
    pub(super) current_buffer_index: Cell<Option<u32>>, //only used for webgl_2
    pub(super) buffer_lookup: BeachMap<DefaultVersion, WebGlBuffer>, 

    pub(super) current_texture_id: Option<Id>,
    pub(super) current_texture_slot: Option<u32>,
    pub(super) texture_lookup: BeachMap<DefaultVersion, WebGlTexture>,
    pub(super) texture_sampler_lookup: Vec<Option<TextureSamplerInfo>>,

    pub(super) extension_lookup: HashMap<String, js_sys::Object>,

    pub(super) current_vao_id: Cell<Option<Id>>,
    pub(super) vao_lookup: BeachMap<DefaultVersion, WebGlVertexArrayObject>,

    pub(super) toggle_flags: ToggleFlags,

    pub(super) func_settings: FuncSettings,

    pub(super) depth_mask: bool
}


impl WebGlRenderer {
    pub fn new(canvas:HtmlCanvasElement, opts:Option<&WebGlContextOptions>) -> Result<Self, Error> {
        let gl = get_webgl_context(&canvas, opts)?;


        let max_texture_units:usize = gl.get_parameter(GlQuery::MaxTextureImageUnits as u32)
            .and_then(|value| {
                      value
                        .as_f64()
                        .map(|val| val as usize)
                        .ok_or(JsValue::null())
            })?;

        //Can't use the vec! macro since TextureSamplerInfo isn't Clone
        let mut texture_sampler_lookup = Vec::with_capacity(max_texture_units);
        for i in 0..max_texture_units {
            texture_sampler_lookup.push(None);
        }


        //The webgl docs don't talk about a default value...
        //seems to be 0 for all - but just in case... it's... set by browser? _shrug_
        let blend_color:Vec<f32> = gl.get_parameter(GlQuery::BlendColor as u32)
                .map(|value| value.into()) //JsValue -> Float32Array
                .map(|value| clone_to_vec_f32(&value))?; //Float32Array -> Vec<f32>
     
        Ok(
            Self {
                gl,
                canvas,

                last_width: 0,
                last_height: 0,

                current_program_id: None, 
                program_lookup: BeachMap::default(),

                current_buffer_id: Cell::new(None),
                current_buffer_target: Cell::new(None), 
                current_buffer_index: Cell::new(None), 
                buffer_lookup: BeachMap::default(),

                current_texture_id: None, 
                current_texture_slot: None,
                texture_lookup: BeachMap::default(),
                texture_sampler_lookup,

                extension_lookup: HashMap::new(),

                current_vao_id: Cell::new(None),
                vao_lookup: BeachMap::default(),

                toggle_flags: ToggleFlags::default(),

                func_settings: FuncSettings{
                    blend_color: (blend_color[0], blend_color[1], blend_color[2], blend_color[3]),
                    ..FuncSettings::default()
                },

                depth_mask: true
            }
        )
    }
}
