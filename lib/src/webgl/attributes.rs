use web_sys::{WebGlProgram};
use crate::errors::{Error, NativeError};
use super::{WebGlRenderer, WebGlContext, DataType, BufferTarget, Id};

//ATTRIBUTES
pub enum AttributeLocation<'a> {
    Name(&'a str),
    Value(u32),
}

pub struct AttributeOptions {
    pub size: i32, 
    pub data_type: DataType,
    pub normalized: bool, 
    pub stride: i32,
    // the WebIDL spec says this is actually a GLintptr or a long long
    // Rust provides functions for either u32 or f64 - and most likely
    // the f64 flavor is to allow the full Number range of JS, i.e. 52 bits
    // However - allowing float values here is probably a more likely source
    // of bugs than allowing > 52 bit values, especially since we're not concerned
    // with safety due to the wasm sandbox
    // So we're allowing the u64 type for larger values and catching accidental floats
    // It's cast to f64 to uploading (which I guess will chop the last 12 bits)
    pub offset: u64 
}

impl AttributeOptions {
    pub fn new(size: i32, data_type: DataType) -> AttributeOptions {
        AttributeOptions {
            size,
            data_type,
            normalized: false,
            stride: 0,
            offset: 0
        }
    }
}

pub fn get_attribute_location_direct(gl:&WebGlContext, program:&WebGlProgram, name:&str) -> Result<u32, Error> {
    Some(gl.get_attrib_location(&program, &name))
        .filter(|x| *x != -1)
        .map(|x| x as u32)
        .ok_or(Error::from(NativeError::AttributeLocation(Some(name.to_owned()))))
}

pub fn activate_attribute_direct(gl:&WebGlContext, loc:u32, opts:&AttributeOptions) {
    gl.vertex_attrib_pointer_with_f64(loc, opts.size, opts.data_type as u32, opts.normalized, opts.stride, opts.offset as f64);
    gl.enable_vertex_attrib_array(loc);
}


impl WebGlRenderer {

    pub fn get_attribute_location_value(&self, name:&str) -> Result<u32, Error> 

    {

        let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self.program_lookup.get(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

        program_info.attribute_lookup
            .get(name)
            .map(|v| *v)
            .ok_or_else(|| Error::from(NativeError::AttributeLocation(Some(name.to_string()))))
    }

    pub fn activate_attribute(&self, loc:&AttributeLocation, opts:&AttributeOptions) -> Result<(), Error> {
        let loc = match loc {
            AttributeLocation::Name(ref name) => {
                self.get_attribute_location_value(&name)?
            },
            AttributeLocation::Value(v) => *v
        };

        activate_attribute_direct(&self.gl, loc, &opts);

        Ok(())
    }

    pub fn activate_buffer_for_attribute(&self, buffer_id:Id, buffer_target:BufferTarget, attribute_loc:&AttributeLocation, opts:&AttributeOptions) -> Result<(), Error> {

        self.activate_buffer(buffer_id, buffer_target)?;
        self.activate_attribute(&attribute_loc, &opts)?;
        Ok(())
    }
}

