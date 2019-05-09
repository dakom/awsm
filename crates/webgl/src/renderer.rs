extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use std::collections::HashMap;
use std::cell::Cell;
use std::cell::RefCell;
use std::cell::Ref;
use std::cell::RefMut;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::VacantEntry;
use std::collections::hash_map::Entry;
use wasm_bindgen::prelude::*;
use web_sys::{console};
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlBuffer, WebGlRenderingContext, WebGlUniformLocation, WebGlTexture};
use super::enums::{DataType, BufferTarget, BufferUsage};
use super::errors::*;
use super::canvas;
use super::extensions;
use super::shader;
use super::attributes;
use super::buffers;
use super::uniforms;
use super::textures;
use wasm_bindgen::JsCast;

type ID = usize;

pub struct WebGlRenderer <'a> {
    pub gl:WebGlRenderingContext,
    pub canvas: HtmlCanvasElement,

    last_width: u32,
    last_height: u32,

    program_info_lookup: RefCell<Vec<Option<ProgramInfo<'a>>>>,
    buffer_lookup: Vec<Option<WebGlBuffer>>,
    extension_lookup: HashMap<&'a str, js_sys::Object>,
    global_attribute_lookup: RefCell<HashMap<&'a str, u32>>,
    texture_lookup: Vec<Option<WebGlTexture>>,

    current_program_id: Cell<Option<ID>>,
    current_buffer_id: Cell<Option<ID>>,
    current_buffer_target: Cell<Option<BufferTarget>>,
    current_texture_id: Cell<Option<ID>>,
    current_texture_slot: Cell<Option<u32>>,
}

struct ProgramInfo <'a> {
    pub id:ID,
    pub program: WebGlProgram,
    pub attribute_lookup: HashMap<&'a str, u32>,
    pub uniform_lookup: HashMap<&'a str, WebGlUniformLocation>
}


impl<'a> Drop for WebGlRenderer<'a> {
    fn drop(&mut self) {
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT); 
        //console::log_1(&JsValue::from_str("Freed GL context!!!"));
    }
}

pub trait WebGlRender {
    fn render(&self, webgl_renderer:&mut WebGlRenderer) -> Result<(), Error>;
}

impl<'a> WebGlRenderer<'a> {
    //Canvas and Context
    pub fn new(canvas: HtmlCanvasElement) -> Result<WebGlRenderer<'a>, Error> {
        canvas::get_canvas_context_1(&canvas)
            .ok_or(Error::from(NativeError::CanvasCreate))
            .map(|gl| WebGlRenderer {
                gl, 
                canvas,
                last_width: 0,
                last_height: 0,
                program_info_lookup: RefCell::new(Vec::new()),
                buffer_lookup: Vec::new(),
                extension_lookup: HashMap::new(),
                global_attribute_lookup: RefCell::new(HashMap::new()),
                texture_lookup: Vec::new(),
                current_program_id: Cell::new(None),
                current_buffer_id: Cell::new(None),
                current_buffer_target: Cell::new(None),
                current_texture_id: Cell::new(None),
                current_texture_slot: Cell::new(None)
            })
    }

    pub fn resize(&mut self, width:u32, height:u32) {
        if self.last_width != width || self.last_height != height {
            let canvas = &mut self.canvas;
            let gl = &mut self.gl;
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

    pub fn compile_program(&mut self, vertex:&str, fragment:&str) -> Result<ID, Error> {
        let program = shader::compile_program(&self.gl, &vertex, &fragment)?;

        let id = {
            let mut program_info_lookup = self.program_info_lookup.borrow_mut();
            let id = program_info_lookup.iter().count();

            let program_info = ProgramInfo {
                id,
                program,
                attribute_lookup: HashMap::new(),
                uniform_lookup: HashMap::new()
            };

            program_info_lookup.push(Some(program_info));

            id
        };

        self.activate_program(id)?;

        Ok(id)
    }

    pub fn activate_program(&self, program_id: ID) -> Result<(), Error> {
        if Some(program_id) != self.current_program_id.get() {
            self.current_program_id.set(Some(program_id));
            let mut program_info_lookup = self.program_info_lookup.borrow_mut();
            let program_id = self.current_program_id.get().ok_or(Error::from(NativeError::MissingShaderProgram))?;
            let program_info = program_info_lookup[program_id].as_mut().ok_or(Error::from(NativeError::MissingShaderProgram))?;

            self.gl.use_program(Some(&program_info.program));
            Ok(())
        } else {
            Ok(())
        }
    }

    //BUFFERS
    pub fn create_buffer(&mut self) -> Result<ID, Error> {

        let buffer = self.gl.create_buffer()
            .ok_or(Error::from(NativeError::NoCreateBuffer))?;

        let id = self.buffer_lookup.iter().count();
        
        self.buffer_lookup.push(Some(buffer));

        Ok(id)
    }

    pub fn activate_buffer(&self, buffer_id:ID, target: BufferTarget) -> Result<(), Error> {

        if Some(buffer_id) != self.current_buffer_id.get() || Some(target) != self.current_buffer_target.get() {
            self.current_buffer_id.set(Some(buffer_id));
            self.current_buffer_target.set(Some(target));

            let buffer = self.get_current_buffer()?; 
            buffers::bind_buffer(&self.gl, target, &buffer);
        }

        Ok(())
    }

    fn get_current_buffer(&self) -> Result<&WebGlBuffer, Error> {
        self.current_buffer_id.get()
            .and_then(|id| self.buffer_lookup[id].as_ref())
            .ok_or(Error::from(NativeError::MissingBuffer))
    }

    pub fn upload_array_buffer(&self, id:ID, values:&[f32], target: BufferTarget, usage:BufferUsage) -> Result<(), Error> {
        self.activate_buffer(id, target)?;

        let buffer = self.get_current_buffer()?; 

        buffers::upload_array_buffer(&self.gl, &values, target, usage, &buffer)
    }

    //ATTRIBUTES
    pub fn get_attribute_location_from_current_program(&self, name:&'a str) -> Result<u32, Error> 
    
    {

        let mut program_info_lookup = self.program_info_lookup.borrow_mut();
        let program_id = self.current_program_id.get().ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = program_info_lookup[program_id].as_mut().ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let entry = program_info.attribute_lookup.entry(&name);

        match entry {
            Entry::Occupied(entry) => Ok(entry.into_mut().clone()),
            Entry::Vacant(entry) => {
               let loc = attributes::get_attribute_location(&self.gl, &program_info.program, &name)?;
               Ok(entry.insert(loc).clone())
            }
        }
    }

    //TODO: pub fn get_attribute_location_from_global(&self, name:&'a str) -> Result<u32, Error> 

    pub fn activate_attribute_loc(&self, loc:u32, opts:&attributes::AttributeOptions) {
        self.gl.vertex_attrib_pointer_with_f64(loc, opts.size, opts.data_type as u32, opts.normalized, opts.stride, opts.offset as f64);
        self.gl.enable_vertex_attrib_array(loc);
    }

    pub fn activate_attribute_name_in_current_program(&self, name:&'a str, opts:&attributes::AttributeOptions) -> Result<(), Error> {
        let loc = self.get_attribute_location_from_current_program(&name)?;

        self.activate_attribute_loc(loc, &opts);

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

    pub fn create_extension_instanced_arrays(&mut self) -> Result<&extensions::AngleInstancedArrays, Error> {
        self.create_extension("ANGLE_instanced_arrays")
            .map(|ext| ext.unchecked_ref::<extensions::AngleInstancedArrays>())
    }

    pub fn get_extension_instanced_arrays(&self) -> Result<&extensions::AngleInstancedArrays, Error> {
        self.get_extension("ANGLE_instanced_arrays")
            .map(|ext| ext.unchecked_ref::<extensions::AngleInstancedArrays>())
    }

    //UNIFORMS
    pub fn get_uniform_loc(&self, name:&'a str) -> Result<WebGlUniformLocation, Error> {

        let mut program_info_lookup = self.program_info_lookup.borrow_mut();
        let program_id = self.current_program_id.get().ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = program_info_lookup[program_id].as_mut().ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let entry = program_info.uniform_lookup.entry(&name);

        match entry {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
               let loc = uniforms::get_uniform_location(&self.gl, &program_info.program, &name)?;
               Ok(entry.insert(loc).clone())
            }
        }
    }

    pub fn set_uniform_name(&self, name:&'a str, data: uniforms::UniformData) -> Result<(), Error> {
        let loc = self.get_uniform_loc(&name)?;
        self.set_uniform_loc(&loc, data);
        Ok(())
    }

    pub fn set_uniform_loc(&self, loc:&WebGlUniformLocation, data: uniforms::UniformData) {
        //TODO Maybe compare to local cache and avoid setting if data hasn't changed?
        uniforms::set_uniform_data(&self.gl, &loc, data);
    }

    pub fn set_uniform_matrix_name(&self, name:&'a str, data: uniforms::UniformMatrixData) -> Result<(), Error> {
        let loc = self.get_uniform_loc(&name)?;
        self.set_uniform_matrix_loc(&loc, data);
        Ok(())
    }

    pub fn set_uniform_matrix_loc(&self, loc:&WebGlUniformLocation, data: uniforms::UniformMatrixData) {
        //TODO Maybe compare to local cache and avoid setting if data hasn't changed?
        uniforms::set_uniform_matrix_data(&self.gl, &loc, data);
    }

    //TEXTURES
    pub fn create_texture(&mut self) -> Result<ID, Error> {
        let texture = self.gl.create_texture().ok_or(Error::from(NativeError::NoCreateTexture))?;

        let id = self.texture_lookup.iter().count();

        self.texture_lookup.push(Some(texture));

        Ok(id)
    }

    //Texture assigning will bind the texture - so the slot for activations effectively becomes None 
    pub fn assign_simple_texture (&self, texture_id:ID, opts:&textures::SimpleTextureOptions, src:&textures::WebGlTextureSource) -> Result<(), Error> {
        let texture = self.texture_lookup[texture_id].as_ref().ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_simple_texture(&self.gl, &opts, &src, &texture)

    }

    pub fn assign_simple_texture_mips (&self, texture_id:ID, opts:&textures::SimpleTextureOptions, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {
        let texture = self.texture_lookup[texture_id].as_ref().ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_simple_texture_mips(&self.gl, &opts, &srcs, &texture)
    }


    pub fn assign_texture (&self, texture_id: ID, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlRenderingContext) -> ()>, src:&textures::WebGlTextureSource) -> Result<(), Error> {
        let texture = self.texture_lookup[texture_id].as_ref().ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_texture(&self.gl, &opts, set_parameters, &src, &texture)
    }

    pub fn assign_texture_mips (&self, texture_id: ID, opts:&textures::TextureOptions, set_parameters:Option<impl Fn(&WebGlRenderingContext) -> ()>, srcs:&[&textures::WebGlTextureSource]) -> Result<(), Error> {
        let texture = self.texture_lookup[texture_id].as_ref().ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id.set(Some(texture_id));
        self.current_texture_slot.set(None);

        textures::assign_texture_mips(&self.gl, &opts, set_parameters, &srcs, &texture)
    }

    //TODO - Texture switching / activation (use current_texture_id)

    //DRAWING
    pub fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        self.gl.draw_arrays(mode, first, count);
    }
    //TODO - blend funcs and stuff
}

