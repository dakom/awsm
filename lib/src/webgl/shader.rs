use std::collections::HashMap;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::VacantEntry;
use std::collections::hash_map::Entry;
use web_sys::{WebGlProgram, WebGlShader, WebGlUniformLocation};
use wasm_bindgen::prelude::JsValue;
use crate::errors::{Error, NativeError};
use crate::helpers::{clone_to_vec_u32};
use super::id::{Id};
use super::{WebGlRenderer, UniformDataType, WebGlContext, GlQuery, UniformBlockQuery, get_attribute_location_direct, get_uniform_location_direct};
use log::{info};

pub struct ProgramInfo {
    pub program: WebGlProgram,
    pub attribute_lookup: HashMap<String, u32>,
    pub uniform_lookup: HashMap<String, WebGlUniformLocation>,
    #[cfg(feature="webgl_2")]
    pub uniform_buffer_loc_lookup: HashMap<String, u32>,
    pub texture_sampler_slot_lookup: HashMap<String, u32>,
}

impl ProgramInfo {
    #[cfg(feature="webgl_1")]
    fn new(program:WebGlProgram) -> Self {
        Self {
            program,
            attribute_lookup: HashMap::new(),
            uniform_lookup: HashMap::new(),
            texture_sampler_slot_lookup: HashMap::new(),
        }
    }

    #[cfg(feature="webgl_2")]
    fn new(program:WebGlProgram) -> Self {
        Self {
            program,
            attribute_lookup: HashMap::new(),
            uniform_lookup: HashMap::new(),
            uniform_buffer_loc_lookup: HashMap::new(),
            texture_sampler_slot_lookup: HashMap::new(),
        }
    }
}


impl WebGlRenderer {
    pub fn compile_program(&mut self, vertex:&str, fragment:&str) -> Result<Id, Error> {
        let program = compile_program(&self.gl, &vertex, &fragment)?;

        let program_info = ProgramInfo::new(program);

        let id = self.program_lookup.insert(program_info);
        
        self.activate_program(id)?;

        self.cache_attribute_ids()?;
        let uniforms_in_blocks = self.cache_uniform_buffers()?;
        let texture_uniforms = self.cache_uniform_ids(&uniforms_in_blocks)?;
        self.bind_texture_uniforms(&texture_uniforms)?;

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

    fn cache_uniform_ids(&mut self, uniforms_in_blocks:&[u32]) -> Result<Vec<String>, Error> {
        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get_mut(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let mut texture_samplers:Vec<String> = Vec::new();
        let max:u32 = self.gl.get_program_parameter(&program_info.program, GlQuery::ActiveUniforms as u32)
            .as_f64()
            .map(|val| val as u32)
            .unwrap_or(0);

        if max <= 0 {
            return Ok(texture_samplers);
        }


        let mut sampler_index = 0;

        for i in (0..max).filter(|n| !uniforms_in_blocks.contains(n)) {
            info!("getting uniform cache info for uniform #{} ", i);
            let (name, type_) = self.gl.get_active_uniform(&program_info.program, i)
                .map(|info| (info.name(), info.type_()))
                .ok_or(Error::from(NativeError::UniformLocation(None)))?;


            let entry = program_info.uniform_lookup.entry(name.to_string());

            match entry {
                Entry::Occupied(entry) => { 
                    info!("skipping uniform cache for [{}] (already exists)", &name);
                },
                Entry::Vacant(entry) => {
                    let loc = get_uniform_location_direct(&self.gl, &program_info.program, &name)?;
                    entry.insert(loc);
                    info!("caching uniform [{}]", &name);
                    match type_ {
                        //Just the sampler types from UniformDataType
                        //matching on enums with casting seems to be a pain point 
                        //(or I missed something in Rust)
                        0x8B5E 
                        | 0x8B60 
                        | 0x8B5F 
                        | 0x8B62 
                        | 0x8DC5 
                        | 0x8DC1 
                        | 0x8DC4 
                        | 0x8DCA 
                        | 0x8DCB 
                        | 0x8DCC 
                        | 0x8DCF 
                        | 0x8DD2 
                        | 0x8DD3 
                        | 0x8DD4 
                        | 0x8DD7 => {
                            texture_samplers.push(name)
                        }
                        _ => {}
                    };
                }
            }
        };

        Ok(texture_samplers)
    }
    fn bind_texture_uniforms(&mut self, texture_uniforms:&[String]) -> Result<(), Error> {
        for (idx, name) in texture_uniforms.iter().enumerate() { 
            self.upload_uniform_ival(&name, idx as i32)?;
        }

        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get_mut(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        for (idx, name) in texture_uniforms.iter().enumerate() { 
            program_info.texture_sampler_slot_lookup.insert(name.to_string(), idx as u32);
        }
        Ok(())
    }
    fn cache_attribute_ids(&mut self) -> Result<(), Error> {
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
                    let loc = get_attribute_location_direct(&self.gl, &program_info.program, &name)?;
                    entry.insert(loc);
                    info!("caching attribute [{}] at location [{}]", &name, loc);
                }
            }
        };

        Ok(())
    }

    #[cfg(feature="webgl_1")]
    fn cache_uniform_block_ids(&mut self) -> Result<Vec<u32>, Error> {
        Ok((vec![]))
    }

    #[cfg(feature="webgl_1")]
    fn cache_uniform_buffers(&mut self) -> Result<Vec<u32>, Error> {
        Ok(Vec::new())
    }

    //returns the uniforms that are in blocks, so they can be excluded from further caching
    #[cfg(feature="webgl_2")]
    fn cache_uniform_buffers(&mut self) -> Result<Vec<u32>, Error> {
        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get_mut(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let mut uniforms_in_blocks:Vec<u32> = Vec::new();

        let max:u32 = self.gl.get_program_parameter(&program_info.program, GlQuery::ActiveUniformBlocks as u32)
            .as_f64()
            .map(|val| val as u32)
            .unwrap_or(0);

        let mut bind_point_offset = self.ubo_global_loc_lookup.len() as u32;

        if max > 0 {
            for i in 0..max {
                let program = &program_info.program;

                let name = self.gl.get_active_uniform_block_name(&program, i).ok_or(Error::from(NativeError::UniformBufferName))?;

                let block_index = self.gl.get_uniform_block_index(&program, &name);

                let global_loc = 
                    self.ubo_global_loc_lookup
                        .iter()
                        .position(|global_name| name == *global_name)
                        .map(|n| n as u32);

                let bind_point = match global_loc { 
                    None => {
                        let ret = bind_point_offset.clone();
                        bind_point_offset += 1;
                        ret
                    },
                    Some(n) => n
                };
                self.gl.uniform_block_binding(&program, block_index, bind_point);
                /*let bind_point = self.gl.get_active_uniform_block_parameter(&program, i, UniformBlockQuery::BindingPoint as u32)?
                        .as_f64().ok_or(Error::from(NativeError::Internal))
                        .map(|val| val as u32)
                        .unwrap();
                */

                let entry = program_info.uniform_buffer_loc_lookup.entry(name.to_string());

                match entry {
                    Entry::Occupied(entry) => { 
                        info!("skipping uniform buffer cache for [{}] (already exists)", &name);
                    },
                    Entry::Vacant(entry) => {
                        entry.insert(bind_point);
                        info!("caching uniform buffer [{}] at index {} and bind point {}", &name, block_index, bind_point);
                    }
                };

                //Need to keep a list of the uniforms that are in blocks, 
                //so they don't get double-cached
                let mut active_uniforms:Vec<u32> = self.gl.get_active_uniform_block_parameter(&program, i, UniformBlockQuery::ActiveUniformIndices as u32)
                    .map(|vals| vals.into())
                    .map(|vals| clone_to_vec_u32(&vals))?;

                uniforms_in_blocks.append(&mut active_uniforms);

            }
        }

        Ok(uniforms_in_blocks)
    }

}

struct CompileSteps {
    program: Option<WebGlProgram>,
    fragment: Option<WebGlShader>,
    vertex: Option<WebGlShader>,
}

type WithError<T> = Result<T,(T,Error)>;

impl CompileSteps {
    pub fn new() -> CompileSteps {
        CompileSteps {
            program: None,
            fragment: None,
            vertex: None,
        }
    }

    pub fn free_shaders(&mut self, gl:&WebGlContext) {
        let free_shader = |s:Option<&WebGlShader>| {
            s.map(|shader:&WebGlShader| {
                //if the shader exists, the program had to have been valid
                gl.detach_shader(self.program.as_ref().unwrap(), shader);
            });
            gl.delete_shader(s);
        };

        free_shader(self.fragment.as_ref()); 
        free_shader(self.vertex.as_ref()); 

        self.fragment = None;
        self.vertex = None;
    }

    pub fn free_all(&mut self, gl:&WebGlContext) {
        self.free_shaders(gl);

        gl.delete_program(self.program.as_ref());
        self.program = None;
    }
}


pub fn compile_program(gl:&WebGlContext, vertex:&str, fragment:&str) -> Result<WebGlProgram, Error> {
    let result = compile_program_steps(&gl, CompileSteps::new())
        .and_then(|compile_steps:CompileSteps|
            compile_source(&gl, compile_steps, fragment, WebGlContext::FRAGMENT_SHADER)
        )
        .and_then(|compile_steps:CompileSteps|
            compile_source(&gl, compile_steps, vertex, WebGlContext::VERTEX_SHADER)
        )
        .and_then(|compile_steps:CompileSteps|
            link_program(&gl, compile_steps)
        );

    match result {
        Ok(mut compile_steps) => {
            compile_steps.free_shaders(gl);
            Ok(compile_steps.program.unwrap())
        }
        Err((mut compile_steps, error_message)) => {
            compile_steps.free_all(gl);
            Err(Error::from(error_message))
        }
    }

}

fn compile_program_steps (gl:&WebGlContext, mut compile_steps:CompileSteps) -> WithError<CompileSteps> { 
    match gl.create_program() {
        Some(program) => {
            compile_steps.program = Some(program);
            Ok(compile_steps)
        }
        None => {
            Err((compile_steps, Error::Native(NativeError::WebGlProgram)))
        }
    }
}

fn compile_source (gl:&WebGlContext, mut compile_steps:CompileSteps, source: &str, source_type:u32) -> WithError<CompileSteps> { 
    let option_shader = gl.create_shader(source_type);

    match option_shader {
        Some(shader) => {
            gl.shader_source(&shader, source);
            gl.compile_shader(&shader);
            match do_with_check( || gl.get_shader_parameter(&shader, WebGlContext::COMPILE_STATUS), || gl.get_shader_info_log(&shader)) {
                Some(error_message) => {
                    Err((compile_steps, Error::from(error_message)))
                }
                None => {
                    gl.attach_shader(&compile_steps.program.as_ref().unwrap(), &shader);
                    if source_type == WebGlContext::VERTEX_SHADER {
                        compile_steps.vertex = Some(shader);
                    } else {
                        compile_steps.fragment = Some(shader);
                    }
                    Ok(compile_steps)
                }
            }

        }
        None => {
            Err((compile_steps, Error::from("bad shader (unknown error")))
        }
    }
}

fn link_program (gl:&WebGlContext, compile_steps:CompileSteps) -> WithError<CompileSteps> { 
    let program = &compile_steps.program.as_ref().unwrap();
    gl.link_program(program);

    match do_with_check( || gl.get_program_parameter(program, WebGlContext::LINK_STATUS), || gl.get_program_info_log(program)) {
        Some(error_message) => {
            Err((compile_steps, Error::from(error_message)))
        }
        None => Ok(compile_steps)
    }
}


fn do_with_check <T,U>(set_status: T, get_status: U) -> Option<String> 
    where T: Fn() -> JsValue, U: Fn() -> Option<String>, 
{

    if set_status() == JsValue::FALSE {
        match get_status() {
            None => Some(String::from("unknown shader compiler error!")),
            err => err
        }
    } else {
        None
    }
}
