use web_sys::{WebGlProgram, WebGlShader};
use wasm_bindgen::prelude::JsValue;
use super::context::{WebGlContext};
use crate::errors::{Error, NativeError};


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
