use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use std::cell::Cell;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::VacantEntry;
use std::collections::hash_map::Entry;
use beach_map::{BeachMap, ID, DefaultVersion};
use web_sys::{WebGlBuffer, WebGlTexture, HtmlCanvasElement, WebGlProgram, WebGlUniformLocation};
use log::{info};
use crate::errors::{Error, NativeError};
use super::context::{WebGlContext, get_context};
use super::buffers;
use super::shader;
use super::attributes;
use super::uniforms;
use super::textures;
use super::extensions;
use super::enums::{ClearBufferMask, TextureTarget, BufferTarget, BufferUsage};

pub type Id = ID<DefaultVersion>; 

pub struct WebGlRenderer <'a> {
    pub gl:WebGlContext,
    pub canvas: HtmlCanvasElement,
    last_width: u32,
    last_height: u32,

    current_program_id: Cell<Option<Id>>,
    program_lookup: BeachMap<DefaultVersion, ProgramInfo<'a>>, 

    current_buffer_id: Cell<Option<Id>>,
    current_buffer_target: Cell<Option<BufferTarget>>,
    buffer_lookup: BeachMap<DefaultVersion, WebGlBuffer>, 

    current_texture_id: Cell<Option<Id>>,
    current_texture_slot: Cell<Option<u32>>,
    texture_lookup: BeachMap<DefaultVersion, WebGlTexture>,
    texture_sampler_lookup: HashMap<u32, TextureSamplerInfo>,

    extension_lookup: HashMap<&'a str, js_sys::Object>,
}

struct ProgramInfo <'a> {
    pub program: WebGlProgram,
    pub attribute_lookup: HashMap<&'a str, u32>,
    pub uniform_lookup: HashMap<&'a str, WebGlUniformLocation>
}

struct TextureSamplerInfo {
    bind_target: TextureTarget,
    texture_id: Id,
}

/*
 * TODO
 * 1. Have cleanup_shaders() to detatch and delete shader
 * 2. Have cleanup_program() to call cleanup_shaders() and also free program
 * 3. See if we can impl Drop for WebGlProgram
 * 4. Setup global attribute locations?
 */

impl <'a> WebGlRenderer <'a> {
    pub fn new(canvas:HtmlCanvasElement) -> Result<Self, Error> {
        let gl = get_context(&canvas)?;

        Ok(
            Self {
                gl,
                canvas,

                last_width: 0,
                last_height: 0,

                current_program_id: Cell::new(None),
                program_lookup: BeachMap::default(),

                current_buffer_id: Cell::new(None),
                current_buffer_target: Cell::new(None),
                buffer_lookup: BeachMap::default(),

                current_texture_id: Cell::new(None),
                current_texture_slot: Cell::new(None),
                texture_lookup: BeachMap::default(),
                texture_sampler_lookup: HashMap::new(),

                extension_lookup: HashMap::new()
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

        Ok(id)
    }

    pub fn activate_program(&self, program_id: Id) -> Result<(), Error> {
        if Some(program_id) != self.current_program_id.get() {
            self.current_program_id.set(Some(program_id));
            let program_id = self.current_program_id.get().ok_or(Error::from(NativeError::MissingShaderProgram))?;
            let program_info = self.program_lookup.get(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

            self.gl.use_program(Some(&program_info.program));
            Ok(())
        } else {
            Ok(())
        }
    }

    fn current_program(&self) -> Result<&ProgramInfo<'a>, Error> {

        let program_id = self.current_program_id.get().ok_or(Error::from(NativeError::MissingShaderProgram))?;
        self.program_lookup.get(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))
    }

    //BUFFERS
    pub fn create_buffer(&mut self) -> Result<Id, Error> {

        let buffer = self.gl.create_buffer()
            .ok_or(Error::from(NativeError::NoCreateBuffer))?;

        let id = self.buffer_lookup.insert(buffer);

        Ok(id)
    }

    pub fn create_buffer_at_attribute_name(&mut self, values:&[f32], buffer_target: BufferTarget, buffer_usage: BufferUsage, attribute_name:&'a str, attribute_opts:&attributes::AttributeOptions) -> Result<Id, Error> {
        let buffer_id = self.create_buffer()?;
        self.upload_buffer(buffer_id, &values, buffer_target, buffer_usage)?;
        self.activate_attribute_name(&attribute_name, &attribute_opts)?;

        Ok(buffer_id)

    }

    pub fn create_buffer_at_attribute_loc(&mut self, values:&[f32], buffer_target: BufferTarget, buffer_usage: BufferUsage, attribute_loc:u32, attribute_opts:&attributes::AttributeOptions) -> Result<Id, Error> {
        let buffer_id = self.create_buffer()?;
        self.upload_buffer(buffer_id, &values, buffer_target, buffer_usage)?;
        self.activate_attribute_loc(attribute_loc, &attribute_opts);

        Ok(buffer_id)
    }

    pub fn activate_buffer(&self, buffer_id:Id, target: BufferTarget) -> Result<(), Error> {

        if Some(buffer_id) != self.current_buffer_id.get() || Some(target) != self.current_buffer_target.get() {
            self.current_buffer_id.set(Some(buffer_id));
            self.current_buffer_target.set(Some(target));

            let buffer = self.get_current_buffer()?; 
            buffers::bind_buffer(&self.gl, target, &buffer);
        }

        Ok(())
    }

    fn get_current_buffer(&self) -> Result<&WebGlBuffer, Error> {
        let buffer_id = self.current_buffer_id.get().ok_or(Error::from(NativeError::MissingBuffer))?;
        self.buffer_lookup.get(buffer_id).ok_or(Error::from(NativeError::MissingShaderProgram))
    }

    pub fn upload_buffer(&self, id:Id, values:&[f32], target: BufferTarget, usage:BufferUsage) -> Result<(), Error> {
        self.activate_buffer(id, target)?;

        let buffer = self.get_current_buffer()?; 

        buffers::upload_buffer(&self.gl, &values, target, usage, &buffer)
    }

    //ATTRIBUTES
    pub fn get_attribute_location(&mut self, name:&'a str) -> Result<u32, Error> 

    {

        let program_id = self.current_program_id.get().ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get_mut(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let entry = program_info.attribute_lookup.entry(&name);

        match entry {
            Entry::Occupied(entry) => Ok(entry.into_mut().clone()),
            Entry::Vacant(entry) => {
                let loc = attributes::get_attribute_location(&self.gl, &program_info.program, &name)?;
                Ok(entry.insert(loc).clone())
            }
        }
    }

    pub fn activate_attribute_loc(&self, loc:u32, opts:&attributes::AttributeOptions) {
        self.gl.vertex_attrib_pointer_with_f64(loc, opts.size, opts.data_type as u32, opts.normalized, opts.stride, opts.offset as f64);
        self.gl.enable_vertex_attrib_array(loc);
    }

    pub fn activate_attribute_name(&mut self, name:&'a str, opts:&attributes::AttributeOptions) -> Result<(), Error> {
        let loc = self.get_attribute_location(&name)?;

        self.activate_attribute_loc(loc, &opts);

        Ok(())
    }

    pub fn set_attribute_name_to_buffer(&mut self, buffer_id:Id, buffer_target: BufferTarget, attribute_name:&'a str, attribute_opts:&attributes::AttributeOptions) -> Result<(), Error> {
        self.activate_buffer(buffer_id, buffer_target)?;
        self.activate_attribute_name(&attribute_name, &attribute_opts)
    }

    pub fn set_attribute_loc_to_buffer(&mut self, buffer_id:Id, buffer_target: BufferTarget, attribute_loc:u32, attribute_opts:&attributes::AttributeOptions) -> Result<(), Error> {
        self.activate_buffer(buffer_id, buffer_target)?;
        self.activate_attribute_loc(attribute_loc, &attribute_opts);

        Ok(())
    }

    //UNIFORMS
    pub fn get_uniform_loc(&mut self, name:&'a str) -> Result<WebGlUniformLocation, Error> {

        let program_id = self.current_program_id.get().ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get_mut(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let entry = program_info.uniform_lookup.entry(&name);

        match entry {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let loc = uniforms::get_uniform_location(&self.gl, &program_info.program, &name)?;
                Ok(entry.insert(loc).clone())
            }
        }
    }

    pub fn set_uniform_name(&mut self, name:&'a str, data: uniforms::UniformData) -> Result<(), Error> {
        let loc = self.get_uniform_loc(&name)?;
        self.set_uniform_loc(&loc, data);
        Ok(())
    }

    pub fn set_uniform_loc(&self, loc:&WebGlUniformLocation, data: uniforms::UniformData) {
        //TODO Maybe compare to local cache and avoid setting if data hasn't changed?
        uniforms::set_uniform_data(&self.gl, &loc, data);
    }

    pub fn set_uniform_matrix_name(&mut self, name:&'a str, data: uniforms::UniformMatrixData) -> Result<(), Error> {
        let loc = self.get_uniform_loc(&name)?;
        self.set_uniform_matrix_loc(&loc, data);
        Ok(())
    }

    pub fn set_uniform_matrix_loc(&self, loc:&WebGlUniformLocation, data: uniforms::UniformMatrixData) {
        //TODO Maybe compare to local cache and avoid setting if data hasn't changed?
        uniforms::set_uniform_matrix_data(&self.gl, &loc, data);
    }

    //TEXTURES
    pub fn create_texture(&mut self) -> Result<Id, Error> {
        let texture = self.gl.create_texture().ok_or(Error::from(NativeError::NoCreateTexture))?;

        let id = self.texture_lookup.insert(texture);

        Ok(id)
    }

    //public interfaces here are simple wrappers to pass the texture target along

    pub fn assign_simple_texture_2d(&self, texture_id:Id, opts:&textures::SimpleTextureOptions, src:&textures::WebGlTextureSource) -> Result<(), Error> {
        self.assign_simple_texture(texture_id, TextureTarget::Texture2D, &opts, &src)
    }
    pub fn assign_simple_texture_mips_2d(&self, texture_id:Id, opts:&textures::SimpleTextureOptions, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {
        self.assign_simple_texture_mips(texture_id, TextureTarget::Texture2D, &opts, &srcs)
    }
    pub fn assign_texture_2d(&self, texture_id: Id, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, src:&textures::WebGlTextureSource) -> Result<(), Error> {
        self.assign_texture(texture_id, TextureTarget::Texture2D, &opts, set_parameters, &src)
    }
    pub fn assign_texture_mips_2d(&self, texture_id: Id, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {
        self.assign_texture_mips(texture_id, TextureTarget::Texture2D, &opts, set_parameters, &srcs)
    }
    pub fn activate_texture_for_sampler_2d(&mut self, texture_id: Id, sampler_index: u32) -> Result<(), Error> {
        self.activate_texture_for_sampler(TextureTarget::Texture2D, texture_id, sampler_index)
    }

    //Texture assigning will bind the texture - so the slot for activations effectively becomes None 
    fn assign_simple_texture(&self, texture_id:Id, bind_target: TextureTarget, opts:&textures::SimpleTextureOptions, src:&textures::WebGlTextureSource) -> Result<(), Error> {
        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_simple_texture(&self.gl, bind_target, &opts, &src, &texture)

    }

    fn assign_simple_texture_mips(&self, texture_id:Id, bind_target: TextureTarget, opts:&textures::SimpleTextureOptions, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_simple_texture_mips(&self.gl, bind_target, &opts, &srcs, &texture)
    }


    fn assign_texture(&self, texture_id: Id, bind_target: TextureTarget, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, src:&textures::WebGlTextureSource) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_texture(&self.gl, bind_target, &opts, set_parameters, &src, &texture)
    }

    fn assign_texture_mips(&self, texture_id: Id, bind_target: TextureTarget, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;
        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_texture_mips(&self.gl, bind_target, &opts, set_parameters, &srcs, &texture)
    }

    fn activate_texture_for_sampler(&mut self, bind_target:TextureTarget, texture_id: Id, sampler_index: u32) -> Result<(), Error> {


        let entry = self.texture_sampler_lookup.entry(sampler_index);

        let requires_activation = match entry {
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                if entry.bind_target != bind_target || entry.texture_id != texture_id {
                    entry.texture_id = texture_id;
                    entry.bind_target = bind_target;
                    true
                } else {
                    false
                }
            },
            Entry::Vacant(entry) => {
                entry.insert(TextureSamplerInfo{
                    texture_id,
                    bind_target
                });
                true
            }
        };

        if requires_activation {
            let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;
            textures::activate_texture_for_sampler(&self.gl, bind_target, sampler_index, &texture);
        }

        Ok(())
    }

    //EXTENSIONS
    fn create_extension(&mut self, name:&'a str) -> Result<&js_sys::Object, Error> {
        if self.extension_lookup.get(&name).is_none() {
            let ext = extensions::get_extension(&self.gl, &name)?;
            self.extension_lookup.insert(&name, ext); 
        }
        self.extension_lookup.get(&name).ok_or(
            Error::from(NativeError::NoExtension)
        )
    }

    fn get_extension(&self, name:&'a str) -> Result<&js_sys::Object, Error> {
        self.extension_lookup.get(&name).ok_or(
            Error::from(NativeError::NoExtension)
        )
    }

    //Not actually used atm because we're defaulting to WebGL2
    //But kept here for a working example of how extensions could be implemented
    pub fn create_extension_instanced_arrays(&mut self) -> Result<&extensions::AngleInstancedArrays, Error> {
        self.create_extension("ANGLE_instanced_arrays")
            .map(|ext| ext.unchecked_ref::<extensions::AngleInstancedArrays>())
    }

    pub fn get_extension_instanced_arrays(&self) -> Result<&extensions::AngleInstancedArrays, Error> {
        self.get_extension("ANGLE_instanced_arrays")
            .map(|ext| ext.unchecked_ref::<extensions::AngleInstancedArrays>())
    }

    //DRAWING
    pub fn clear(&self, bits: &[ClearBufferMask]) {
        let mut combined = 0u32;
        for bit in bits {
            combined = combined | *bit as u32;
        }
        self.gl.clear(combined);
    }

    pub fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        self.gl.draw_arrays(mode, first, count);
    }
}
