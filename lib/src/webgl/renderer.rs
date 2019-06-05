use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlUniformLocation};
use log::{info};
use crate::errors::{Error, NativeError};
use super::context::{WebGlContext, get_context};
use super::shader;
use slotmap::{DefaultKey, SlotMap};

//TODO - https://www.reddit.com/r/rust/comments/9s0hbk/slotmap_03_released_support_for_custom_key_types/
//Can we store ProgramInfo even though it's non-copy? Maybe as secondary Map? 
pub type ID = DefaultKey; 

pub struct WebGlRenderer <'a> {
    pub gl:WebGlContext,
    pub canvas: HtmlCanvasElement,
    last_width: u32,
    last_height: u32,

    program_info_lookup: SlotMap<ID, ProgramInfo<'a>>, 
}

struct ProgramInfo <'a> {
    pub id:ID,
    pub program: WebGlProgram,
    pub attribute_lookup: HashMap<&'a str, u32>,
    pub uniform_lookup: HashMap<&'a str, WebGlUniformLocation>
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
                last_height: 0
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
}
