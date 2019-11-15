use super::funcs::FuncSettings;
use super::misc::MiscSettings;
use super::toggles::ToggleFlags;
use super::{ BufferTarget, GlQuery, Id, ProgramInfo, TextureInfo, WebGlCommon };
use crate::errors::Error;
use beach_map::{BeachMap, DefaultVersion};
use rustc_hash::FxHashMap;
use std::cell::Cell;
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlVertexArrayObject};
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

pub type WebGl1Renderer = WebGlRenderer<WebGlRenderingContext>;
pub type WebGl2Renderer = WebGlRenderer<WebGl2RenderingContext>;

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

pub struct WebGlRenderer<T: WebGlCommon> {
    pub gl: T,
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
    pub(super) texture_lookup: BeachMap<DefaultVersion, TextureInfo>,
    pub(super) texture_sampler_lookup: Vec<Option<Id>>,

    pub(super) extension_lookup: FxHashMap<String, js_sys::Object>,

    pub(super) ubo_global_loc_lookup: Vec<String>,

    pub(super) current_vao_id: Cell<Option<Id>>,
    pub(super) vao_lookup: BeachMap<DefaultVersion, WebGlVertexArrayObject>,

    pub(super) toggle_flags: ToggleFlags,

    pub(super) func_settings: FuncSettings,
    pub(super) misc_settings: MiscSettings,
}

impl<T: WebGlCommon> WebGlRenderer<T> {
    pub fn new(gl: T) -> Result<Self, Error> {
        let canvas = gl.awsm_get_canvas()?;

        let max_texture_units: usize =
            gl.awsm_get_parameter_usize(GlQuery::MaxTextureImageUnits)?;

        //Can't use the vec! macro since TextureSamplerInfo isn't Clone
        let mut texture_sampler_lookup = Vec::with_capacity(max_texture_units);
        for _ in 0..max_texture_units {
            texture_sampler_lookup.push(None);
        }

        //The webgl docs don't talk about a default value...
        //seems to be 0 for all - but just in case... it's... set by browser? _shrug_
        let blend_color: Vec<f32> = gl.awsm_get_parameter_vf32(GlQuery::BlendColor)?;

        Ok(Self {
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
            texture_lookup: BeachMap::default(),
            texture_sampler_lookup,

            extension_lookup: FxHashMap::default(),

            current_vao_id: Cell::new(None),
            vao_lookup: BeachMap::default(),

            ubo_global_loc_lookup: Vec::new(),

            toggle_flags: ToggleFlags::default(),

            func_settings: FuncSettings {
                blend_color: (
                    blend_color[0],
                    blend_color[1],
                    blend_color[2],
                    blend_color[3],
                ),
                ..FuncSettings::default()
            },

            misc_settings: MiscSettings::default(),
        })
    }
}
