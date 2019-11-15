use super::id::Id;
use super::{
    ProgramQuery, ShaderQuery, ShaderType, UniformBlockActiveQuery, UniformBlockQuery,
    WebGlCommon, WebGlRenderer,
};
use crate::data::{clone_to_vec_u32};
use crate::errors::{Error, NativeError};
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;
use web_sys::{WebGl2RenderingContext, WebGlActiveInfo, WebGlRenderingContext};
use web_sys::{WebGlProgram, WebGlShader, WebGlUniformLocation};

pub struct ProgramInfo {
    pub program: WebGlProgram,
    pub attribute_lookup: FxHashMap<String, u32>,
    pub uniform_lookup: FxHashMap<String, WebGlUniformLocation>,
    pub texture_sampler_slot_lookup: FxHashMap<String, u32>,

    //only needed for webgl2
    pub uniform_buffer_loc_lookup: FxHashMap<String, u32>,
    pub uniform_buffer_offset_lookup: FxHashMap<String, FxHashMap<String, u32>>,
}

impl ProgramInfo {
    fn new(program: WebGlProgram) -> Self {
        Self {
            program,
            attribute_lookup: FxHashMap::default(),
            uniform_lookup: FxHashMap::default(),
            texture_sampler_slot_lookup: FxHashMap::default(),
            uniform_buffer_loc_lookup: FxHashMap::default(),
            uniform_buffer_offset_lookup: FxHashMap::default(),
        }
    }
}

pub trait PartialWebGlShaders {
    fn awsm_create_program(&self) -> Result<WebGlProgram, Error>;
    fn awsm_create_shader(&self, type_: ShaderType) -> Option<WebGlShader>;
    fn awsm_attach_shader(&self, program: &WebGlProgram, shader: &WebGlShader);
    fn awsm_detach_shader(&self, program: &WebGlProgram, shader: &WebGlShader);
    fn awsm_delete_shader(&self, shader: &WebGlShader);
    fn awsm_delete_program(&self, program: &WebGlProgram);
    fn awsm_shader_source(&self, shader: &WebGlShader, source: &str);
    fn awsm_compile_shader(&self, shader: &WebGlShader);
    fn awsm_link_program(&self, program: &WebGlProgram);
    fn awsm_get_shader_parameter_bool(
        &self,
        shader: &WebGlShader,
        query: ShaderQuery,
    ) -> Result<bool, Error>;
    fn awsm_get_program_parameter_bool(
        &self,
        program: &WebGlProgram,
        query: ProgramQuery,
    ) -> Result<bool, Error>;
    fn awsm_get_program_parameter_u32(
        &self,
        program: &WebGlProgram,
        query: ProgramQuery,
    ) -> Result<u32, Error>;
    fn awsm_get_shader_info_log(&self, shader: &WebGlShader) -> Option<String>;
    fn awsm_get_program_info_log(&self, program: &WebGlProgram) -> Option<String>;
    fn awsm_use_program(&self, program: &WebGlProgram);
    fn awsm_get_active_uniform(
        &self,
        program: &WebGlProgram,
        index: u32,
    ) -> Result<WebGlActiveInfo, Error>;
    fn awsm_get_active_attrib(
        &self,
        program: &WebGlProgram,
        index: u32,
    ) -> Result<WebGlActiveInfo, Error>;
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlShaders for $type {
            //Put all the common methods here:

            fn awsm_create_program(&self) -> Result<WebGlProgram, Error> {
                self.create_program().ok_or(Error::Native(NativeError::WebGlProgram))
            }

            fn awsm_create_shader(&self, type_: ShaderType) -> Option<WebGlShader> {
                self.create_shader(type_ as u32)
            }

            fn awsm_attach_shader(&self, program: &WebGlProgram, shader: &WebGlShader) {
                self.attach_shader(program, shader);
            }

            fn awsm_detach_shader(&self, program: &WebGlProgram, shader: &WebGlShader) {
                self.detach_shader(program, shader);
            }

            fn awsm_delete_shader(&self, shader: &WebGlShader) {
                self.delete_shader(Some(shader));
            }

            fn awsm_delete_program(&self, program: &WebGlProgram) {
                self.delete_program(Some(program));
            }

            fn awsm_shader_source(&self, shader: &WebGlShader, source: &str) {
                self.shader_source(shader, source);
            }

            fn awsm_compile_shader(&self, shader:&WebGlShader) {
                self.compile_shader(shader);
            }

            fn awsm_link_program(&self, program: &WebGlProgram) {
                self.link_program(program);
            }

            fn awsm_get_shader_parameter_bool(&self, shader: &WebGlShader, query: ShaderQuery) -> Result<bool, Error> {
                self.get_shader_parameter(shader, query as u32)
                    .as_bool()
                    .ok_or(Error::from(NativeError::JsValueExpectedBool))
            }

            fn awsm_get_program_parameter_bool(&self, program: &WebGlProgram, query: ProgramQuery) -> Result<bool, Error> {
                self.get_program_parameter(program, query as u32)
                    .as_bool()
                    .ok_or(Error::from(NativeError::JsValueExpectedBool))
            }

            fn awsm_get_program_parameter_u32(&self, program:&WebGlProgram, query:ProgramQuery) -> Result<u32, Error> {
                let number = self.get_program_parameter(program, query as u32)
                    .as_f64()
                    .ok_or(Error::from(NativeError::JsValueExpectedNumber))?;

                Ok(number as u32)
            }

            fn awsm_get_shader_info_log(&self, shader: &WebGlShader) -> Option<String> {
                self.get_shader_info_log(shader)
            }

            fn awsm_get_program_info_log(&self, program: &WebGlProgram) -> Option<String> {
                self.get_program_info_log(program)
            }

            fn awsm_use_program(&self, program: &WebGlProgram) {
                self.use_program(Some(program))
            }

            fn awsm_get_active_uniform( &self, program: &WebGlProgram, index: u32) -> Result<WebGlActiveInfo, Error> {
                self.get_active_uniform(program, index)
                    .ok_or(Error::from(NativeError::UniformLocation(None)))
            }

            fn awsm_get_active_attrib(&self, program: &WebGlProgram, index: u32) -> Result<WebGlActiveInfo, Error> {
                self.get_active_attrib(program, index)
                    .ok_or(Error::from(NativeError::AttributeLocation(None)))
            }

            $($defs)*
        })+
    };
}

impl_context! {
    WebGlRenderingContext{}
    WebGl2RenderingContext{}
}

impl<T: WebGlCommon> WebGlRenderer<T> {
    pub fn activate_program(&mut self, program_id: Id) -> Result<(), Error> {
        if Some(program_id) != self.current_program_id {
            self.current_program_id = Some(program_id);
            let program_info = self
                .program_lookup
                .get(program_id)
                .ok_or(Error::from(NativeError::MissingShaderProgram))?;
            self.gl.awsm_use_program(&program_info.program);
            Ok(())
        } else {
            Ok(())
        }
    }

    fn cache_uniform_ids(&mut self, uniforms_in_blocks: &[u32]) -> Result<Vec<String>, Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self
            .program_lookup
            .get_mut(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let mut texture_samplers: Vec<String> = Vec::new();
        let max: u32 = self
            .gl
            .awsm_get_program_parameter_u32(&program_info.program, ProgramQuery::ActiveUniforms)
            .unwrap_or(0);

        if max <= 0 {
            return Ok(texture_samplers);
        }

        for i in (0..max).filter(|n| !uniforms_in_blocks.contains(n)) {

            #[cfg(feature = "debug_log")]
            log::info!("getting uniform cache info for uniform #{} ", i);
            let (name, type_) = self
                .gl
                .awsm_get_active_uniform(&program_info.program, i)
                .map(|info| (info.name(), info.type_()))?;

            let entry = program_info.uniform_lookup.entry(name.to_string());

            match entry {
                Entry::Occupied(_) => {
                    #[cfg(feature = "debug_log")]
                    log::info!("skipping uniform cache for [{}] (already exists)", &name);
                }
                Entry::Vacant(entry) => {
                    let loc = self
                        .gl
                        .awsm_get_uniform_location(&program_info.program, &name)?;
                    entry.insert(loc);
                    #[cfg(feature = "debug_log")]
                    log::info!("caching uniform [{}]", &name);
                    match type_ {
                        //Just the sampler types from UniformDataType
                        //matching on enums with casting seems to be a pain point
                        //(or I missed something in Rust)
                        0x8B5E | 0x8B60 | 0x8B5F | 0x8B62 | 0x8DC5 | 0x8DC1 | 0x8DC4 | 0x8DCA
                        | 0x8DCB | 0x8DCC | 0x8DCF | 0x8DD2 | 0x8DD3 | 0x8DD4 | 0x8DD7 => {
                            texture_samplers.push(name)
                        }
                        _ => {}
                    };
                }
            }
        }

        Ok(texture_samplers)
    }
    fn bind_texture_uniforms(&mut self, texture_uniforms: &[String]) -> Result<(), Error> {
        for (idx, name) in texture_uniforms.iter().enumerate() {
            self.upload_uniform_ival(&name, idx as i32)?;
        }

        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self
            .program_lookup
            .get_mut(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        for (idx, name) in texture_uniforms.iter().enumerate() {
            program_info
                .texture_sampler_slot_lookup
                .insert(name.to_string(), idx as u32);
        }
        Ok(())
    }
    fn cache_attribute_ids(&mut self) -> Result<(), Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self
            .program_lookup
            .get_mut(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let max: u32 = self
            .gl
            .awsm_get_program_parameter_u32(&program_info.program, ProgramQuery::ActiveAttributes)
            .unwrap_or(0);

        if max <= 0 {
            return Ok(());
        }

        for i in 0..max {
            let name = self
                .gl
                .awsm_get_active_attrib(&program_info.program, i)
                .map(|info| info.name())?;

            let entry = program_info.attribute_lookup.entry(name.to_string());

            match entry {
                Entry::Occupied(_) => {
                    #[cfg(feature = "debug_log")]
                    log::info!("skipping attribute cache for [{}] (already exists)", &name);
                }
                Entry::Vacant(entry) => {
                    let loc = self
                        .gl
                        .awsm_get_attribute_location(&program_info.program, &name)?;
                    entry.insert(loc);
                    #[cfg(feature = "debug_log")]
                    log::info!("caching attribute [{}] at location [{}]", &name, loc);
                }
            }
        }

        Ok(())
    }
}

impl WebGlRenderer<WebGlRenderingContext> {
    pub fn compile_program(&mut self, vertex: &str, fragment: &str) -> Result<Id, Error> {
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
    fn cache_uniform_buffers(&mut self) -> Result<Vec<u32>, Error> {
        Ok(Vec::new())
    }
}

impl WebGlRenderer<WebGl2RenderingContext> {
    pub fn compile_program(&mut self, vertex: &str, fragment: &str) -> Result<Id, Error> {
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

    //returns the uniforms that are in blocks, so they can be excluded from further caching
    fn cache_uniform_buffers(&mut self) -> Result<Vec<u32>, Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self
            .program_lookup
            .get_mut(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        let mut uniforms_in_blocks: Vec<u32> = Vec::new();

        let max: u32 = self
            .gl
            .awsm_get_program_parameter_u32(
                &program_info.program,
                ProgramQuery::ActiveUniformBlocks,
            )
            .unwrap_or(0);

        let mut max_bind_point_offset = self.ubo_global_loc_lookup.len() as u32;

        if max > 0 {
            for i in 0..max {
                let program = &program_info.program;

                let name = self
                    .gl
                    .get_active_uniform_block_name(&program, i)
                    .ok_or(Error::from(NativeError::UniformBufferName))?;

                let block_index = self.gl.get_uniform_block_index(&program, &name);

                let global_loc = self
                    .ubo_global_loc_lookup
                    .iter()
                    .position(|global_name| name == *global_name)
                    .map(|n| n as u32);

                let bind_point = match global_loc {
                    None => {
                        let ret = max_bind_point_offset.clone();
                        max_bind_point_offset += 1;
                        ret
                    }
                    Some(n) => n,
                };
                self.gl
                    .uniform_block_binding(&program, block_index, bind_point);

                //program_info.uniform_buffer_offset_lookup
                /*let bind_point = self.gl.get_active_uniform_block_parameter(&program, i, UniformBlockQuery::BindingPoint as u32)?
                        .as_f64().ok_or(Error::from(NativeError::Internal))
                        .map(|val| val as u32)
                        .unwrap();
                */

                let entry = program_info
                    .uniform_buffer_loc_lookup
                    .entry(name.to_string());

                match entry {
                    Entry::Occupied(_) => {

                        #[cfg(feature = "debug_log")]
                        log::info!(
                            "skipping uniform buffer cache for [{}] (already exists)",
                            &name
                        );
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(bind_point);
                        #[cfg(feature = "debug_log")]
                        log::info!(
                            "caching uniform buffer [{}] at index {} and bind point {}",
                            &name, block_index, bind_point
                        );
                    }
                };

                //Need to keep a list of the uniforms that are in blocks,
                //so they don't get double-cached
                let active_uniforms: Vec<u32> = self
                    .gl
                    .get_active_uniform_block_parameter(
                        &program,
                        i,
                        UniformBlockQuery::ActiveUniformIndices as u32,
                    )
                    .map(|vals| vals.into())
                    .map(|vals| clone_to_vec_u32(&vals))?;

                uniforms_in_blocks.extend(&active_uniforms);

                //Also need to cache their offsets
                let block_lookup = program_info
                    .uniform_buffer_offset_lookup
                    .entry(name.to_string())
                    .or_insert_with(|| FxHashMap::default());

                let offsets: Vec<u32> = unsafe {
                    let values = js_sys::Uint32Array::view(&active_uniforms);
                    let active_uniform_offsets = self.gl.get_active_uniforms(
                        &program,
                        &values,
                        UniformBlockActiveQuery::Offset as u32,
                    );
                    clone_to_vec_u32(&active_uniform_offsets.into())
                };

                #[cfg(feature = "debug_log")]
                log::info!("{:?}", &offsets);

                for (idx, loc) in active_uniforms.iter().enumerate() {
                    let (u_name, _u_type_, _u_size) = self
                        .gl
                        .get_active_uniform(&program_info.program, *loc)
                        .map(|info| (info.name(), info.type_(), info.size()))
                        .ok_or(Error::from(NativeError::UniformLocation(None)))?;

                    let offset = offsets[idx];

                    block_lookup.insert(u_name.clone(), offset);

                    #[cfg(feature = "debug_log")]
                    log::info!("uniform {} in block {} has offset {}", u_name, name, offset);
                }
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

type WithError<T> = Result<T, (T, Error)>;

impl CompileSteps {
    pub fn new() -> CompileSteps {
        CompileSteps {
            program: None,
            fragment: None,
            vertex: None,
        }
    }

    pub fn free_shaders<T: WebGlCommon>(&mut self, gl: &T) {
        let free_shader = |s: Option<&WebGlShader>| {
            s.map(|shader: &WebGlShader| {
                //if the shader exists, the program had to have been valid
                gl.awsm_detach_shader(self.program.as_ref().unwrap(), shader);
                gl.awsm_delete_shader(shader);
            });
        };

        free_shader(self.fragment.as_ref());
        free_shader(self.vertex.as_ref());

        self.fragment = None;
        self.vertex = None;
    }

    pub fn free_all<T: WebGlCommon>(&mut self, gl: &T) {
        self.free_shaders(gl);

        self.program.as_ref().map(|program: &WebGlProgram| {
            gl.awsm_delete_program(program);
        });

        self.program = None;
    }
}

pub fn compile_program<T: WebGlCommon>(
    gl: &T,
    vertex: &str,
    fragment: &str,
) -> Result<WebGlProgram, Error> {
    let result = compile_program_steps(gl, CompileSteps::new())
        .and_then(|compile_steps: CompileSteps| {
            compile_source(gl, compile_steps, fragment, ShaderType::Fragment)
        })
        .and_then(|compile_steps: CompileSteps| {
            compile_source(gl, compile_steps, vertex, ShaderType::Vertex)
        })
        .and_then(|compile_steps: CompileSteps| link_program(gl, compile_steps));

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

fn compile_program_steps<T: WebGlCommon>(
    gl: &T,
    mut compile_steps: CompileSteps,
) -> WithError<CompileSteps> {
    match gl.awsm_create_program() {
        Ok(program) => {
            compile_steps.program = Some(program);
            Ok(compile_steps)
        }
        Err(err) => Err((compile_steps, err)),
    }
}

fn compile_source<T: WebGlCommon>(
    gl: &T,
    mut compile_steps: CompileSteps,
    source: &str,
    source_type: ShaderType,
) -> WithError<CompileSteps> {
    let option_shader = gl.awsm_create_shader(source_type);

    match option_shader {
        Some(shader) => {
            gl.awsm_shader_source(&shader, source);
            gl.awsm_compile_shader(&shader);
            match do_with_check(
                || gl.awsm_get_shader_parameter_bool(&shader, ShaderQuery::CompileStatus),
                || gl.awsm_get_shader_info_log(&shader),
            ) {
                Some(error_message) => Err((compile_steps, Error::from(error_message))),
                None => {
                    gl.awsm_attach_shader(&compile_steps.program.as_ref().unwrap(), &shader);
                    if source_type == ShaderType::Vertex {
                        compile_steps.vertex = Some(shader);
                    } else {
                        compile_steps.fragment = Some(shader);
                    }
                    Ok(compile_steps)
                }
            }
        }
        None => Err((compile_steps, Error::from("bad shader (unknown error"))),
    }
}

fn link_program<T: WebGlCommon>(gl: &T, compile_steps: CompileSteps) -> WithError<CompileSteps> {
    let program = &compile_steps.program.as_ref().unwrap();
    gl.awsm_link_program(program);

    match do_with_check(
        || gl.awsm_get_program_parameter_bool(program, ProgramQuery::LinkStatus),
        || gl.awsm_get_program_info_log(program),
    ) {
        Some(error_message) => Err((compile_steps, Error::from(error_message))),
        None => Ok(compile_steps),
    }
}

fn do_with_check<T, U>(set_status: T, get_status: U) -> Option<String>
where
    T: Fn() -> Result<bool, Error>,
    U: Fn() -> Option<String>,
{
    match set_status() {
        Ok(flag) => {
            if !flag {
                match get_status() {
                    None => Some(String::from("unknown shader compiler error!")),
                    err => err,
                }
            } else {
                None
            }
        }

        Err(err) => Some(err.to_string()),
    }
}
