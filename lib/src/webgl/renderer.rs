use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use std::cell::Cell;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::VacantEntry;
use std::collections::hash_map::Entry;
use beach_map::{BeachMap, ID, DefaultVersion};
use web_sys::{console, WebGlVertexArrayObject, WebGlBuffer, WebGlTexture, HtmlCanvasElement, WebGlProgram, WebGlUniformLocation};
use log::{info};
use crate::errors::{Error, NativeError};
use super::context::{WebGlContext, get_context};
use super::buffers;
use super::shader;
use super::attributes;
use super::uniforms;
use super::textures;
use super::extensions;
use super::enums::{CmpFunction, BlendEquation, BlendFactor, GlQuery, DataType, BeginMode, GlToggle, ClearBufferMask, TextureTarget, BufferTarget, BufferUsage};

#[cfg(feature = "webgl_1")]
use web_sys::{OesVertexArrayObject, AngleInstancedArrays};

pub type Id = ID<DefaultVersion>; 

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
    last_width: u32,
    last_height: u32,

    current_program_id: Option<Id>,
    program_lookup: BeachMap<DefaultVersion, ProgramInfo>, 

    current_buffer_id: Cell<Option<Id>>,
    current_buffer_target: Cell<Option<BufferTarget>>,
    buffer_lookup: BeachMap<DefaultVersion, WebGlBuffer>, 

    current_texture_id: Option<Id>,
    current_texture_slot: Option<u32>,
    texture_lookup: BeachMap<DefaultVersion, WebGlTexture>,
    texture_sampler_lookup: Vec<Option<TextureSamplerInfo>>,

    extension_lookup: HashMap<String, js_sys::Object>,

    current_vao_id: Cell<Option<Id>>,
    vao_lookup: BeachMap<DefaultVersion, WebGlVertexArrayObject>,

    toggle_flags: ToggleFlags,

    func_settings: FuncSettings
}

struct ToggleFlags {
    blend: bool,
    cull_face: bool,
    depth_test: bool,
    dither: bool,
    polygon_offset_fill: bool,
    sample_alpha_to_coverage: bool,
    sample_coverage: bool,
    scissor_test: bool, 
    stencil_test: bool,
    rasterizer_discard: bool,
}

struct FuncSettings {
    depth_func: CmpFunction,
    blend_func: (BlendFactor, BlendFactor),
    blend_func_separate: (BlendFactor, BlendFactor, BlendFactor, BlendFactor),
    blend_equation: BlendEquation, 
    blend_equation_separate: (BlendEquation, BlendEquation) 
}

struct ProgramInfo {
    pub program: WebGlProgram,
    pub attribute_lookup: HashMap<String, u32>,
    pub uniform_lookup: HashMap<String, WebGlUniformLocation>
}

struct TextureSamplerInfo {
    bind_target: TextureTarget,
    texture_id: Id,
}

pub struct VertexArray<'a> {
    pub loc: AttributeLocation<'a>,
    pub buffer_id: Id,
    pub opts: &'a attributes::AttributeOptions
}

impl <'a> VertexArray<'a> {
    pub fn new(loc:AttributeLocation<'a>, buffer_id: Id, opts: &'a attributes::AttributeOptions) -> Self {
        Self {
            loc,
            buffer_id,
            opts
        }
    }
}

pub enum AttributeLocation<'a> {
    Name(&'a str),
    Value(u32),
}

pub enum UniformLocation<'a> {
    Name(&'a str),
    Value(WebGlUniformLocation),
}

/*
 * TODO
 * 1. Have cleanup_shaders() to detatch and delete shader
 * 2. Have cleanup_program() to call cleanup_shaders() and also free program
 * 3. See if we can impl Drop for WebGlProgram
 * 4. Setup global attribute locations?
 */

impl WebGlRenderer {
    pub fn new(canvas:HtmlCanvasElement) -> Result<Self, Error> {
        let gl = get_context(&canvas)?;


        let max_texture_units:usize = gl.get_parameter(GlQuery::MaxTextureImageUnits as u32)
            .and_then(|value| {
                      value
                        .as_f64()
                        .map(|val| val as usize)
                        .ok_or(JsValue::null())
            })?;

        info!("Max texture units: {}", max_texture_units);

        //Can't use the vec! macro since TextureSamplerInfo isn't Clone
        let mut texture_sampler_lookup = Vec::with_capacity(max_texture_units);
        for i in 0..max_texture_units {
            texture_sampler_lookup.push(None);
        }
        
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
                buffer_lookup: BeachMap::default(),

                current_texture_id: None, 
                current_texture_slot: None,
                texture_lookup: BeachMap::default(),
                texture_sampler_lookup,

                extension_lookup: HashMap::new(),

                current_vao_id: Cell::new(None),
                vao_lookup: BeachMap::default(),

                toggle_flags: ToggleFlags {
                    blend: false,
                    cull_face: false,
                    depth_test: false,
                    dither: false,
                    polygon_offset_fill: false,
                    sample_alpha_to_coverage: false,
                    sample_coverage: false,
                    scissor_test: false, 
                    stencil_test: false,
                    rasterizer_discard: false,
                },

                func_settings: FuncSettings {
                    depth_func: CmpFunction::Less,
                    blend_func: (BlendFactor::One, BlendFactor::Zero),
                    blend_func_separate: (BlendFactor::One, BlendFactor::Zero, BlendFactor::One, BlendFactor::Zero),
                    blend_equation: BlendEquation::Add,
                    blend_equation_separate: (BlendEquation::Add, BlendEquation::Add) 
                }
            }
        )
    }

    pub fn resize(&mut self, width:u32, height:u32) {
        if self.last_width != width || self.last_height != height {
            info!("resizing: {},{}", width, height);

            let gl = &mut self.gl;
            let canvas = &mut self.canvas;
            canvas.set_width(width);
            canvas.set_height(height);
            gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());
            self.last_width = width;
            self.last_height = height;
        }
    }

    pub fn current_size(&self) -> (u32, u32) {
        (self.last_width, self.last_height)
    }

    //SHADERS

    pub fn compile_program(&mut self, vertex:&str, fragment:&str) -> Result<Id, Error> {
        let program = shader::compile_program(&self.gl, &vertex, &fragment)?;

        let program_info = ProgramInfo {
            program,
            attribute_lookup: HashMap::new(),
            uniform_lookup: HashMap::new()
        };

        let id = self.program_lookup.insert(program_info);
        
        self.activate_program(id)?;

        self.cache_attributes()?;
        self.cache_uniforms()?;

        Ok(id)
    }

    pub fn activate_program(&mut self, program_id: Id) -> Result<(), Error> {
        if Some(program_id) != self.current_program_id {
            self.current_program_id = Some(program_id);
            let program_info = self.program_lookup.get(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;
            self.gl.use_program(Some(&program_info.program));
            Ok(())
        } else {
            Ok(())
        }
    }


    //BUFFERS
    pub fn create_buffer(&mut self) -> Result<Id, Error> {
        let buffer = self.gl.create_buffer()
            .ok_or(Error::from(NativeError::NoCreateBuffer))?;

        let id = self.buffer_lookup.insert(buffer);

        Ok(id)
    }


    fn _activate_buffer_nocheck(&self, buffer_id:Id, target: BufferTarget) -> Result<(), Error> {

        self.current_buffer_id.set(Some(buffer_id));
        self.current_buffer_target.set(Some(target));

        let buffer = self.get_current_buffer()?; 
        buffers::bind_buffer(&self.gl, target, &buffer);

        Ok(())
    }
    pub fn activate_buffer(&self, buffer_id:Id, target: BufferTarget) -> Result<(), Error> {

        if Some(buffer_id) != self.current_buffer_id.get() || Some(target) != self.current_buffer_target.get() {
            self._activate_buffer_nocheck(buffer_id, target)
        } else {
            Ok(())
        }
    }

    fn get_current_buffer(&self) -> Result<&WebGlBuffer, Error> {
        let buffer_id = self.current_buffer_id.get().ok_or(Error::from(NativeError::MissingBuffer))?;
        self.buffer_lookup.get(buffer_id).ok_or(Error::from(NativeError::MissingShaderProgram))
    }

    pub fn upload_buffer_f32(&self, id:Id, values:&[f32], target: BufferTarget, usage:BufferUsage) -> Result<(), Error> {
        self.activate_buffer(id, target)?;

        let buffer = self.get_current_buffer()?; 

        buffers::upload_buffer_f32(&self.gl, &values, target, usage, &buffer)
    }

    pub fn upload_buffer_u8(&self, id:Id, values:&[u8], target: BufferTarget, usage:BufferUsage) -> Result<(), Error> {
        self.activate_buffer(id, target)?;

        let buffer = self.get_current_buffer()?; 

        buffers::upload_buffer_u8(&self.gl, &values, target, usage, &buffer)
    }

    //Just some helpers to make it simpler
    pub fn upload_buffer_f32_to_attribute(&self, id:Id, values:&[f32], target: BufferTarget, usage:BufferUsage, attribute_loc:&AttributeLocation, opts:&attributes::AttributeOptions) -> Result<(), Error> {
        self.upload_buffer_f32(id, &values, target, usage)?;
        self.activate_attribute(&attribute_loc, &opts)?;
        Ok(())
    }

    pub fn upload_buffer_u8_to_attribute(&self, id:Id, values:&[u8], target: BufferTarget, usage:BufferUsage, attribute_loc:&AttributeLocation, opts:&attributes::AttributeOptions) -> Result<(), Error> {
        self.upload_buffer_u8(id, &values, target, usage)?;
        self.activate_attribute(&attribute_loc, &opts)?;
        Ok(())
    }

    //ATTRIBUTES

    fn cache_attributes(&mut self) -> Result<(), Error> {
        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get_mut(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let max:u32 = self.gl.get_program_parameter(&program_info.program, GlQuery::ActiveAttributes as u32)
            .as_f64()
            .map(|val| val as u32)
            .unwrap_or(0);

        if(max <= 0) {
            return Ok(());
        }

        for i in 0..max {
            let name = self.gl.get_active_attrib(&program_info.program, i)
                .map(|info| info.name())
                .ok_or(Error::from(NativeError::AttributeLocation(None)))?;

            let entry = program_info.attribute_lookup.entry(name.to_string());

            match entry {
                Entry::Occupied(entry) => { 
                    info!("skipping attribute cache for [{}] (already exists)", &name);
                },
                Entry::Vacant(entry) => {
                    let loc = attributes::get_attribute_location(&self.gl, &program_info.program, &name)?;
                    entry.insert(loc);
                    info!("caching attribute [{}] at location [{}]", &name, loc);
                }
            }
        };

        Ok(())
    }

    pub fn get_attribute_location_value(&self, name:&str) -> Result<u32, Error> 

    {

        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        program_info.attribute_lookup
            .get(name)
            .map(|v| *v)
            .ok_or_else(|| Error::from(NativeError::AttributeLocation(Some(name.to_string()))))
    }

    pub fn activate_attribute(&self, loc:&AttributeLocation, opts:&attributes::AttributeOptions) -> Result<(), Error> {
        let loc = match loc {
            AttributeLocation::Name(ref name) => {
                self.get_attribute_location_value(&name)?
            },
            AttributeLocation::Value(v) => *v
        };

        self.gl.vertex_attrib_pointer_with_f64(loc, opts.size, opts.data_type as u32, opts.normalized, opts.stride, opts.offset as f64);
        self.gl.enable_vertex_attrib_array(loc);

        Ok(())
    }

    pub fn activate_buffer_for_attribute(&self, buffer_id:Id, buffer_target:BufferTarget, attribute_loc:&AttributeLocation, opts:&attributes::AttributeOptions) -> Result<(), Error> {

        self.activate_buffer(buffer_id, buffer_target)?;
        self.activate_attribute(&attribute_loc, &opts)?;
        Ok(())
    }

    //UNIFORMS
    fn cache_uniforms(&mut self) -> Result<(), Error> {
        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get_mut(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let max:u32 = self.gl.get_program_parameter(&program_info.program, GlQuery::ActiveUniforms as u32)
            .as_f64()
            .map(|val| val as u32)
            .unwrap_or(0);

        if(max <= 0) {
            return Ok(());
        }

        for i in 0..max {
            let name = self.gl.get_active_uniform(&program_info.program, i)
                .map(|info| info.name())
                .ok_or(Error::from(NativeError::UniformLocation(None)))?;

            let entry = program_info.uniform_lookup.entry(name.to_string());

            match entry {
                Entry::Occupied(entry) => { 
                    info!("skipping uniform cache for [{}] (already exists)", &name);
                },
                Entry::Vacant(entry) => {
                    let loc = uniforms::get_uniform_location(&self.gl, &program_info.program, &name)?;
                    entry.insert(loc);
                    info!("caching uniform [{}]", &name);
                }
            }
        };

        Ok(())
    }
    pub fn get_uniform_location_value(&self, name:&str) -> Result<WebGlUniformLocation, Error> {

        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        program_info.uniform_lookup
            .get(name)
            .map(|v| v.clone())
            .ok_or_else(|| Error::from(NativeError::UniformLocation(Some(name.to_string()))))
    }

    pub fn upload_uniform<U>(&self, loc:&UniformLocation, data:&U) -> Result<(), Error> 
    where U: uniforms::UniformData
    {
        //TODO Maybe compare to local cache and avoid setting if data hasn't changed?
        let loc = match loc {
            UniformLocation::Name(ref name) => {
                let loc = self.get_uniform_location_value(&name)?;
                data.upload(&self.gl, &loc);
            },
            UniformLocation::Value(ref loc) => {
                data.upload(&self.gl, &loc);
            }
        };
        Ok(())
    }

    //TEXTURES
    pub fn create_texture(&mut self) -> Result<Id, Error> {
        let texture = self.gl.create_texture().ok_or(Error::from(NativeError::NoCreateTexture))?;

        let id = self.texture_lookup.insert(texture);

        Ok(id)
    }

    //public interfaces here are simple wrappers to pass the texture target along

    pub fn assign_simple_texture(&mut self, texture_id:Id, opts:&textures::SimpleTextureOptions, src:&textures::WebGlTextureSource) -> Result<(), Error> {
        self.assign_simple_texture_target(texture_id, TextureTarget::Texture2D, &opts, &src)
    }
    pub fn assign_simple_texture_mips(&mut self, texture_id:Id, opts:&textures::SimpleTextureOptions, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {
        self.assign_simple_texture_mips_target(texture_id, TextureTarget::Texture2D, &opts, &srcs)
    }
    pub fn assign_texture(&mut self, texture_id: Id, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, src:&textures::WebGlTextureSource) -> Result<(), Error> {
        self.assign_texture_target(texture_id, TextureTarget::Texture2D, &opts, set_parameters, &src)
    }
    pub fn assign_texture_mips(&mut self, texture_id: Id, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {
        self.assign_texture_mips_target(texture_id, TextureTarget::Texture2D, &opts, set_parameters, &srcs)
    }
    pub fn activate_texture_for_sampler(&mut self, texture_id: Id, sampler_index: usize) -> Result<(), Error> {
        self.activate_texture_for_sampler_target(TextureTarget::Texture2D, texture_id, sampler_index)
    }

    //Texture assigning will bind the texture - so the slot for activations effectively becomes None 
    fn assign_simple_texture_target(&mut self, texture_id:Id, bind_target: TextureTarget, opts:&textures::SimpleTextureOptions, src:&textures::WebGlTextureSource) -> Result<(), Error> {
        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id = Some(texture_id);
        self.current_texture_slot = None;

        textures::assign_simple_texture_target(&self.gl, bind_target, &opts, &src, &texture)

    }

    fn assign_simple_texture_mips_target(&mut self, texture_id:Id, bind_target: TextureTarget, opts:&textures::SimpleTextureOptions, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id = Some(texture_id);
        self.current_texture_slot = None;

        textures::assign_simple_texture_mips_target(&self.gl, bind_target, &opts, &srcs, &texture)
    }


    fn assign_texture_target(&mut self, texture_id: Id, bind_target: TextureTarget, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, src:&textures::WebGlTextureSource) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id = Some(texture_id);
        self.current_texture_slot = None;

        textures::assign_texture_target(&self.gl, bind_target, &opts, set_parameters, &src, &texture)
    }

    fn assign_texture_mips_target(&mut self, texture_id: Id, bind_target: TextureTarget, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;
        self.current_texture_id = Some(texture_id);
        self.current_texture_slot = None;

        textures::assign_texture_mips_target(&self.gl, bind_target, &opts, set_parameters, &srcs, &texture)
    }

    fn activate_texture_for_sampler_target(&mut self, bind_target:TextureTarget, texture_id: Id, sampler_index: usize) -> Result<(), Error> {


        let entry = self.texture_sampler_lookup.get(sampler_index).ok_or(Error::from(NativeError::Internal))?;

        let requires_activation = match entry {
            Some(entry) => {
                if entry.bind_target != bind_target || entry.texture_id != texture_id {
                    true
                } else {
                    false
                }
            },
            None => {
                true
            }
        };

        if requires_activation {
            self.texture_sampler_lookup[sampler_index] = Some(TextureSamplerInfo{
                texture_id,
                bind_target
            });
            let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;
            textures::activate_texture_for_sampler_target(&self.gl, bind_target, sampler_index, &texture);
        }

        Ok(())
    }

    //EXTENSIONS
    pub fn register_extension(&mut self, name:&str) -> Result<&js_sys::Object, Error> {
        if self.extension_lookup.get(name).is_none() {
            let ext = extensions::get_extension(&self.gl, &name)?;
            self.extension_lookup.insert(name.to_string(), ext); 
        }
        self.extension_lookup.get(name).ok_or(
            Error::from(NativeError::NoExtension)
        )
    }

    fn get_extension(&self, name:&str) -> Result<&js_sys::Object, Error> {
        self.extension_lookup.get(name).ok_or(
            Error::from(NativeError::NoExtension)
        )
    }


    //INSTANCING - WebGL 1 requires extension, WebGL 2 is native

    #[cfg(feature = "webgl_1")]
    pub fn register_extension_instanced_arrays(&mut self) -> Result<&AngleInstancedArrays, Error> {
        self.register_extension("ANGLE_instanced_arrays")
            .map(|ext| ext.unchecked_ref::<AngleInstancedArrays>())
    }
    #[cfg(feature = "webgl_1")]
    pub fn get_extension_instanced_arrays(&self) -> Result<&AngleInstancedArrays, Error> {
        self.get_extension("ANGLE_instanced_arrays")
            .map(|ext| ext.unchecked_ref::<AngleInstancedArrays>())
    }

    #[cfg(feature = "webgl_1")]
    pub fn vertex_attrib_divisor(&self, loc: u32, divisor: u32) -> Result<(), Error> {
        let ext = self.get_extension_instanced_arrays()?;
        ext.vertex_attrib_divisor_angle(loc, divisor);
        Ok(())
    }
    #[cfg(feature = "webgl_2")]
    pub fn vertex_attrib_divisor(&self, loc: u32, divisor: u32) -> Result<(), Error> {
        self.gl.vertex_attrib_divisor(loc, divisor);
        Ok(())
    }

    #[cfg(feature = "webgl_1")]
    pub fn draw_arrays_instanced(&self, mode: BeginMode, first: u32, count: u32, primcount: u32) -> Result<(), Error> {
        let ext = self.get_extension_instanced_arrays()?;
        ext.draw_arrays_instanced_angle(mode as u32, first as i32, count as i32, primcount as i32);
        Ok(())
    }
    #[cfg(feature = "webgl_2")]
    pub fn draw_arrays_instanced(&self, mode: BeginMode, first: u32, count: u32, primcount: u32) -> Result<(), Error> {
        self.gl.draw_arrays_instanced( mode as u32, first as i32, count as i32, primcount as i32);
        Ok(())
    }

    #[cfg(feature = "webgl_1")]
    pub fn draw_elements_instanced(&self, mode: BeginMode, count: u32, data_type: DataType, offset: u32, primcount: u32) -> Result<(), Error> {
        let ext = self.get_extension_instanced_arrays()?;
        ext.draw_elements_instanced_angle_with_i32(mode as u32, count as i32, data_type as u32, offset as i32, primcount as i32);
        Ok(())
    }
    #[cfg(feature = "webgl_2")]
    pub fn draw_elements_instanced(&mut self, mode: BeginMode, count: u32, data_type: DataType, offset: u32, primcount: u32) -> Result<(), Error> {
        self.gl.draw_elements_instanced_with_i32( mode as u32, count as i32, data_type as u32, offset as i32, primcount as i32);
        Ok(())
    }

    //VERTEX ARRAYS
    //
    #[cfg(feature = "webgl_1")]
    pub fn register_extension_vertex_array(&mut self) -> Result<&OesVertexArrayObject, Error> {
        self.register_extension("OES_vertex_array_object")
            .map(|ext| ext.unchecked_ref::<OesVertexArrayObject>())
    }
    #[cfg(feature = "webgl_1")]
    fn _get_extension_vertex_array(&self) -> Result<&OesVertexArrayObject, Error> {
        self.get_extension("OES_vertex_array_object")
            .map(|ext| ext.unchecked_ref::<OesVertexArrayObject>())
    }

    #[cfg(feature = "webgl_1")]
    fn _bind_vertex_array(&self, id:Option<Id>, vao:Option<&WebGlVertexArrayObject>) -> Result<(), Error> {
        let ext = self._get_extension_vertex_array()?;
        ext.bind_vertex_array_oes(vao);
        self.current_vao_id.set(id);
        Ok(())
    }

    #[cfg(feature = "webgl_2")]
    fn _bind_vertex_array(&self, id:Option<Id>, vao:Option<&WebGlVertexArrayObject>) -> Result<(), Error> {
        self.gl.bind_vertex_array(vao);
        self.current_vao_id.set(id);
        Ok(())
    }

    #[cfg(feature = "webgl_1")]
    fn _create_vertex_array(&mut self) -> Result<Id, Error> {
        let ext = self._get_extension_vertex_array()?;
        let vao = ext.create_vertex_array_oes().ok_or(Error::from(NativeError::VertexArrayCreate))?;
        let id = self.vao_lookup.insert(vao);
        Ok(id)
    }

    #[cfg(feature = "webgl_2")]
    fn _create_vertex_array(&mut self) -> Result<Id, Error> {
        let vao = self.gl.create_vertex_array().ok_or(Error::from(NativeError::VertexArrayCreate))?;
        let id = self.vao_lookup.insert(vao);
        Ok(id)
    }

    pub fn create_vertex_array(&mut self) -> Result<Id, Error> {
        self._create_vertex_array()
    }

    pub fn release_vertex_array(&self) -> Result<(), Error> {
        self._bind_vertex_array(None, None)
    }


    pub fn activate_vertex_array(&self, vao_id:Id) -> Result<(), Error> {
        if Some(vao_id) != self.current_vao_id.get() {
            if let Some(vao) = self.vao_lookup.get(vao_id) { 
                self._bind_vertex_array(Some(vao_id), Some(&vao));
            } else {
                return Err(Error::from(NativeError::VertexArrayMissing));
            }
        }
        Ok(())
    }

    pub fn assign_vertex_array(&self, vao_id:Id, element_buffer_id:Option<Id>, configs:&[VertexArray]) -> Result<(), Error> {
        let result = if let Some(vao) = self.vao_lookup.get(vao_id) { 
            self._bind_vertex_array(Some(vao_id), Some(&vao));

            //Skip buffer assignment cache checks
            if let Some(element_buffer_id) = element_buffer_id {
                self._activate_buffer_nocheck(element_buffer_id, BufferTarget::ElementArrayBuffer)?;
            }

            for config in configs {
                self._activate_buffer_nocheck(config.buffer_id, BufferTarget::ArrayBuffer)?;
                self.activate_attribute(&config.loc, &config.opts)?;
            }
            Ok(())
        } else {
            Err(Error::from(NativeError::VertexArrayMissing))
        };
           
        //relase it for the next call that might use elements
        self.release_vertex_array();

        result
    }
    
    //TOGGLES
    pub fn toggle(&mut self, toggle:GlToggle, flag:bool) {
    
        let curr = match toggle {
            GlToggle::Blend => &mut self.toggle_flags.blend,
            GlToggle::CullFace => &mut self.toggle_flags.cull_face,
            GlToggle::DepthTest => &mut self.toggle_flags.depth_test,
            GlToggle::Dither => &mut self.toggle_flags.dither,
            GlToggle::PolygonOffsetFill => &mut self.toggle_flags.polygon_offset_fill,
            GlToggle::SampleAlphaToCoverage => &mut self.toggle_flags.sample_alpha_to_coverage,
            GlToggle::SampleCoverage => &mut self.toggle_flags.sample_coverage,
            GlToggle::ScissorTest => &mut self.toggle_flags.scissor_test,
            GlToggle::StencilTest => &mut self.toggle_flags.stencil_test,
            GlToggle::RasterizerDiscard => &mut self.toggle_flags.rasterizer_discard,
        };
       
        if *curr != flag {
            *curr = flag;
            let toggle = toggle as u32;
            match flag {
                true => self.gl.enable(toggle),
                false => self.gl.disable(toggle)
            };
        }
    }

    //FUNCS
    pub fn set_depth_func(&mut self, func:CmpFunction) {
        if self.func_settings.depth_func != func {
            self.gl.depth_func(func as u32);
            self.func_settings.depth_func = func;
        }
    }
    pub fn set_blend_func(&mut self, sfactor: BlendFactor, dfactor: BlendFactor) {

        let curr = self.func_settings.blend_func;

        if curr.0 != sfactor || curr.1 != dfactor { 
            self.gl.blend_func(sfactor as u32, dfactor as u32);
            self.func_settings.blend_func = (sfactor, dfactor);
        }
    }
    pub fn set_blend_func_separate(&mut self, src_rgb: BlendFactor, dest_rgb: BlendFactor, src_alpha:BlendFactor, dest_alpha: BlendFactor) {

        let curr = self.func_settings.blend_func_separate;

        if curr.0 != src_rgb || curr.1 != dest_rgb || curr.2 != src_alpha || curr.3 != dest_alpha { 
            self.gl.blend_func_separate(src_rgb as u32, dest_rgb as u32, src_alpha as u32, dest_alpha as u32);
            self.func_settings.blend_func_separate = (src_rgb, dest_rgb, src_alpha, dest_alpha);
        }
    }
    pub fn set_blend_equation(&mut self, mode:BlendEquation) {
        if self.func_settings.blend_equation != mode {
            self.gl.blend_equation(mode as u32);
            self.func_settings.blend_equation = mode ;
        }
    }
    pub fn set_blend_equation_separate(&mut self, rgb_mode:BlendEquation, alpha_mode:BlendEquation) {

        let curr = self.func_settings.blend_equation_separate;

        if curr.0 != rgb_mode || curr.1 != alpha_mode { 
            self.gl.blend_equation_separate(rgb_mode as u32, alpha_mode as u32);
            self.func_settings.blend_equation_separate = (rgb_mode, alpha_mode);
        }
    }

    //DRAWING
    pub fn clear(&self, bits: &[ClearBufferMask]) {
        let mut combined = 0u32;
        for bit in bits {
            combined = combined | *bit as u32;
        }
        self.gl.clear(combined);
    }

    pub fn draw_arrays(&self, mode: BeginMode, first: u32, count: u32) {
        self.gl.draw_arrays(mode as u32, first as i32, count as i32);
    }

    pub fn draw_elements(&self, mode: BeginMode, count: u32, data_type:DataType, offset:u32) {
        self.gl.draw_elements_with_i32(mode as u32, count as i32, data_type as u32, offset as i32);
    }
}
