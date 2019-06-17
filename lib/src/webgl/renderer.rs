use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use std::cell::Cell;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::VacantEntry;
use std::collections::hash_map::Entry;
use beach_map::{BeachMap, ID, DefaultVersion};
use web_sys::{WebGlVertexArrayObject, WebGlBuffer, WebGlTexture, HtmlCanvasElement, WebGlProgram, WebGlUniformLocation};
use log::{info};
use crate::errors::{Error, NativeError};
use super::context::{WebGlContext, get_context};
use super::buffers;
use super::shader;
use super::attributes;
use super::uniforms;
use super::textures;
use super::extensions;
use super::enums::{DataType, BeginMode, GlToggle, ClearBufferMask, TextureTarget, BufferTarget, BufferUsage};

pub type Id = ID<DefaultVersion>; 


pub struct WebGlRenderer {
    pub gl:WebGlContext,
    pub canvas: HtmlCanvasElement,
    last_width: u32,
    last_height: u32,

    current_program_id: Cell<Option<Id>>,
    program_lookup: BeachMap<DefaultVersion, ProgramInfo>, 

    current_buffer_id: Cell<Option<Id>>,
    current_buffer_target: Cell<Option<BufferTarget>>,
    buffer_lookup: BeachMap<DefaultVersion, WebGlBuffer>, 

    current_texture_id: Cell<Option<Id>>,
    current_texture_slot: Cell<Option<u32>>,
    texture_lookup: BeachMap<DefaultVersion, WebGlTexture>,
    texture_sampler_lookup: HashMap<u32, TextureSamplerInfo>,

    extension_lookup: HashMap<String, js_sys::Object>,
    
    toggle_lookup: HashMap<u32, bool>,

    current_vao_id: Cell<Option<Id>>,
    vao_lookup: BeachMap<DefaultVersion, WebGlVertexArrayObject>
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

                extension_lookup: HashMap::new(),
                toggle_lookup: HashMap::new(),

                current_vao_id: Cell::new(None), 
                vao_lookup: BeachMap::default(),
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
    pub fn upload_buffer_f32_to_attribute(&mut self, id:Id, values:&[f32], target: BufferTarget, usage:BufferUsage, attribute_loc:&AttributeLocation, opts:&attributes::AttributeOptions) -> Result<(), Error> {
        self.upload_buffer_f32(id, &values, target, usage)?;
        self.activate_attribute(&attribute_loc, &opts)?;
        Ok(())
    }

    pub fn upload_buffer_u8_to_attribute(&mut self, id:Id, values:&[u8], target: BufferTarget, usage:BufferUsage, attribute_loc:&AttributeLocation, opts:&attributes::AttributeOptions) -> Result<(), Error> {
        self.upload_buffer_u8(id, &values, target, usage)?;
        self.activate_attribute(&attribute_loc, &opts)?;
        Ok(())
    }

    //ATTRIBUTES
    pub fn get_attribute_location_value(&mut self, name:&str) -> Result<u32, Error> 

    {

        let program_id = self.current_program_id.get().ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get_mut(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        match program_info.attribute_lookup.get(name) {
            Some(value) => Ok(*value),
            None => {
                let loc = attributes::get_attribute_location(&self.gl, &program_info.program, &name)?;
                program_info.attribute_lookup.insert(name.to_string(), loc);
                Ok(loc)
            }
        }
    }

    pub fn activate_attribute(&mut self, loc:&AttributeLocation, opts:&attributes::AttributeOptions) -> Result<(), Error> {
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

    pub fn activate_buffer_for_attribute(&mut self, buffer_id:Id, buffer_target:BufferTarget, attribute_loc:&AttributeLocation, opts:&attributes::AttributeOptions) -> Result<(), Error> {

        self.activate_buffer(buffer_id, buffer_target)?;
        self.activate_attribute(&attribute_loc, &opts)?;
        Ok(())
    }

    //UNIFORMS
    pub fn get_uniform_location_value(&mut self, name:&str) -> Result<WebGlUniformLocation, Error> {

        let program_id = self.current_program_id.get().ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get_mut(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        match program_info.uniform_lookup.get(name) {
            Some(value) => Ok(value.clone()),
            None => {
                let loc = uniforms::get_uniform_location(&self.gl, &program_info.program, &name)?;
                program_info.uniform_lookup.insert(name.to_string(), loc.clone());
                Ok(loc)
            }
        }
    }

    pub fn upload_uniform<U>(&mut self, loc:&UniformLocation, data:&U) -> Result<(), Error> 
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

    pub fn assign_simple_texture(&self, texture_id:Id, opts:&textures::SimpleTextureOptions, src:&textures::WebGlTextureSource) -> Result<(), Error> {
        self.assign_simple_texture_target(texture_id, TextureTarget::Texture2D, &opts, &src)
    }
    pub fn assign_simple_texture_mips(&self, texture_id:Id, opts:&textures::SimpleTextureOptions, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {
        self.assign_simple_texture_mips_target(texture_id, TextureTarget::Texture2D, &opts, &srcs)
    }
    pub fn assign_texture(&self, texture_id: Id, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, src:&textures::WebGlTextureSource) -> Result<(), Error> {
        self.assign_texture_target(texture_id, TextureTarget::Texture2D, &opts, set_parameters, &src)
    }
    pub fn assign_texture_mips(&self, texture_id: Id, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {
        self.assign_texture_mips_target(texture_id, TextureTarget::Texture2D, &opts, set_parameters, &srcs)
    }
    pub fn activate_texture_for_sampler(&mut self, texture_id: Id, sampler_index: u32) -> Result<(), Error> {
        self.activate_texture_for_sampler_target(TextureTarget::Texture2D, texture_id, sampler_index)
    }

    //Texture assigning will bind the texture - so the slot for activations effectively becomes None 
    fn assign_simple_texture_target(&self, texture_id:Id, bind_target: TextureTarget, opts:&textures::SimpleTextureOptions, src:&textures::WebGlTextureSource) -> Result<(), Error> {
        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_simple_texture_target(&self.gl, bind_target, &opts, &src, &texture)

    }

    fn assign_simple_texture_mips_target(&self, texture_id:Id, bind_target: TextureTarget, opts:&textures::SimpleTextureOptions, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_simple_texture_mips_target(&self.gl, bind_target, &opts, &srcs, &texture)
    }


    fn assign_texture_target(&self, texture_id: Id, bind_target: TextureTarget, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, src:&textures::WebGlTextureSource) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_texture_target(&self.gl, bind_target, &opts, set_parameters, &src, &texture)
    }

    fn assign_texture_mips_target(&self, texture_id: Id, bind_target: TextureTarget, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;
        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_texture_mips_target(&self.gl, bind_target, &opts, set_parameters, &srcs, &texture)
    }

    fn activate_texture_for_sampler_target(&mut self, bind_target:TextureTarget, texture_id: Id, sampler_index: u32) -> Result<(), Error> {


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
            textures::activate_texture_for_sampler_target(&self.gl, bind_target, sampler_index, &texture);
        }

        Ok(())
    }

    //EXTENSIONS
    fn create_extension(&mut self, name:&str) -> Result<&js_sys::Object, Error> {
        if self.extension_lookup.get(name).is_none() {
            let ext = extensions::get_extension(&self.gl, &name)?;
            self.extension_lookup.insert(name.to_string(), ext); 
        }
        self.extension_lookup.get(name).ok_or(
            Error::from(NativeError::NoExtension)
        )
    }


    //INSTANCING - WebGL 1 requires extension, WebGL 2 is native

    #[cfg(feature = "webgl_1")]
    pub fn get_extension_instanced_arrays(&mut self) -> Result<&extensions::AngleInstancedArrays, Error> {
        self.create_extension("ANGLE_instanced_arrays")
            .map(|ext| ext.unchecked_ref::<extensions::AngleInstancedArrays>())
    }

    #[cfg(feature = "webgl_1")]
    pub fn vertex_attrib_divisor(&mut self, loc: u32, divisor: u32) -> Result<(), Error> {
        let ext = self.get_extension_instanced_arrays()?;
        ext.vertex_attrib_divisor_angle(loc, divisor);
        Ok(())
    }

    #[cfg(feature = "webgl_1")]
    pub fn draw_arrays_instanced(&mut self, mode: BeginMode, first: u32, count: u32, primcount: u32) -> Result<(), Error> {
        let ext = self.get_extension_instanced_arrays()?;
        ext.draw_arrays_instanced_angle(mode as u32, first as i32, count as i32, primcount as i32).map_err(|err| err.into())
    }
    #[cfg(feature = "webgl_1")]
    pub fn draw_elements_instanced(&mut self, mode: BeginMode, count: u32, data_type: DataType, offset: u32, primcount: u32) -> Result<(), Error> {
        let ext = self.get_extension_instanced_arrays()?;
        ext.draw_elements_instanced_angle(mode as u32, count as i32, data_type as u32, offset as i32, primcount as i32).map_err(|err| err.into())
    }
    
    #[cfg(feature = "webgl_2")]
    pub fn vertex_attrib_divisor(&self, loc: u32, divisor: u32) -> Result<(), Error> {
        self.gl.vertex_attrib_divisor(loc, divisor);
        Ok(())
    }

    #[cfg(feature = "webgl_2")]
    pub fn draw_arrays_instanced(&self, mode: BeginMode, first: u32, count: u32, primcount: u32) -> Result<(), Error> {
        self.gl.draw_arrays_instanced( mode as u32, first as i32, count as i32, primcount as i32);
        Ok(())
    }
    #[cfg(feature = "webgl_2")]
    pub fn draw_elements_instanced(&mut self, mode: BeginMode, count: u32, data_type: DataType, offset: u32, primcount: u32) -> Result<(), Error> {
        self.gl.draw_elements_instanced_with_i32( mode as u32, count as i32, data_type as u32, offset as i32, primcount as i32);
        Ok(())
    }

    //VERTEX ARRAYS
    //
    #[cfg(feature = "webgl_2")]
    pub fn bind_vertex_array(&self, vao:Option<&WebGlVertexArrayObject>) {
        self.gl.bind_vertex_array(vao);
    }

    #[cfg(feature = "webgl_2")]
    pub fn _create_vertex_array(&self) -> Option<WebGlVertexArrayObject> {
        self.gl.create_vertex_array()
    }

    pub fn create_vertex_array(&mut self) -> Result<Id, Error> {
        let vao = self._create_vertex_array().ok_or(Error::from(NativeError::VertexArrayCreate))?;

        let id = self.vao_lookup.insert(vao);
        Ok(id)
    }

    pub fn activate_vertex_array(&self, vao_id:Id) -> Result<(), Error> {
            if let Some(vao) = self.vao_lookup.get(vao_id) { 

                self.bind_vertex_array(Some(&vao));
                self.current_vao_id.set(Some(vao_id));
                Ok(())
            } else {
                Err(Error::from(NativeError::VertexArrayMissing))
            }
        /*
        if Some(vao_id) != self.current_vao_id.get() {
            if let Some(vao) = self.vao_lookup.get(vao_id) { 

                self.bind_vertex_array(Some(&vao));
                self.current_vao_id.set(Some(vao_id));
            } else {
                return Err(Error::from(NativeError::VertexArrayMissing));
            }
        }
        Ok(())
        */
    }

    pub fn release_vertex_array(&self) {
        self.bind_vertex_array(None);
        self.current_vao_id.set(None);
    }

    pub fn assign_vertex_array(&mut self, vao_id:Id, element_buffer_id:Option<Id>, configs:&[VertexArray]) -> Result<(), Error> {
        let result = if let Some(vao) = self.vao_lookup.get(vao_id) { 
            self.bind_vertex_array(Some(&vao));

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
            
        self.release_vertex_array();

        result
    }
    
    //TOGGLES
    pub fn toggle(&mut self, toggle:GlToggle, flag:bool) {
       
        let toggle = toggle as u32;

        let entry = self.toggle_lookup.entry(toggle);

        let requires_activation = match entry {
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                if *entry != flag {
                    *entry = flag;
                    true
                } else {
                    false
                }
            },
            Entry::Vacant(entry) => {
                entry.insert(flag);
                true
            }
        };

        if requires_activation {
            match flag {
                true => self.gl.enable(toggle),
                false => self.gl.disable(toggle)
            }
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
