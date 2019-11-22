use super::{BufferDataImpl, BufferTarget, DataType, Id, WebGlCommon, WebGlRenderer};
use crate::errors::{Error, NativeError};
use web_sys::WebGlProgram;
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

//ATTRIBUTES
#[derive(Debug)]
pub struct AttributeOptions {
    pub size: u8, //according to spec, must be 1,2,3,4
    pub data_type: DataType,
    pub normalized: bool,
    pub stride: u8, //according to spec, can't be larger than 255
    // the WebIDL spec says this is actually a GLintptr or a long long
    // Rust provides functions for either u32 or f64 - and most likely
    // the f64 flavor is to allow the full Number range of JS, i.e. 52 bits
    // However - allowing float values here is probably a more likely source
    // of bugs than allowing > 52 bit values, especially since we're not concerned
    // with safety due to the wasm sandbox
    // So we're allowing the u64 type for larger values and catching accidental floats
    // It's cast to f64 to uploading (which I guess will chop the last 12 bits)
    pub offset: u64,
}

impl AttributeOptions {
    pub fn new(size: u8, data_type: DataType) -> AttributeOptions {
        AttributeOptions {
            size,
            data_type,
            normalized: false,
            stride: 0,
            offset: 0,
        }
    }
}

pub trait PartialWebGlAttribute {
    fn awsm_get_attribute_location(&self, program: &WebGlProgram, name: &str)
        -> Result<u32, Error>;
    fn awsm_activate_attribute(&self, loc: u32, opts: &AttributeOptions);
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlAttribute for $type {
            //Put all the common methods here:
            fn awsm_get_attribute_location(&self, program:&WebGlProgram, name:&str) -> Result<u32, Error> {
                Some(self.get_attrib_location(&program, &name))
                    .filter(|x| *x != -1)
                    .map(|x| x as u32)
                    .ok_or(Error::from(NativeError::AttributeLocation(Some(name.to_owned()))))
            }

            fn awsm_activate_attribute(&self, loc:u32, opts:&AttributeOptions) {
                self.vertex_attrib_pointer_with_f64(loc, opts.size as i32, opts.data_type as u32, opts.normalized, opts.stride as i32, opts.offset as f64);
                self.enable_vertex_attrib_array(loc);
            }

            $($defs)*
        })+
    };
}

impl_context! {
    WebGlRenderingContext{}
    WebGl2RenderingContext{}
}

//The attribute lookups are cached at shader compilation (see shader.rs)
impl<T: WebGlCommon> WebGlRenderer<T> {
    pub fn get_attribute_location_value(&self, name: &str) -> Result<u32, Error> {
        let program_id = self
            .current_program_id
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;
        let program_info = self
            .program_lookup
            .get(program_id)
            .ok_or(Error::from(NativeError::MissingShaderProgram))?;

        program_info
            .attribute_lookup
            .get(name)
            .map(|v| *v)
            .ok_or_else(|| Error::from(NativeError::AttributeLocation(Some(name.to_string()))))
    }

    pub fn activate_attribute_loc(&self, target_loc: u32, opts: &AttributeOptions) {
        self.gl.awsm_activate_attribute(target_loc, &opts);
    }
    //convenience helpers
    pub fn activate_attribute(
        &self,
        target_name: &str,
        opts: &AttributeOptions,
    ) -> Result<(), Error> {
        let loc = self.get_attribute_location_value(&target_name)?;
        self.gl.awsm_activate_attribute(loc, &opts);
        Ok(())
    }

    pub fn activate_buffer_for_attribute(
        &self,
        buffer_id: Id,
        buffer_target: BufferTarget,
        attribute_name: &str,
        opts: &AttributeOptions,
    ) -> Result<(), Error> {
        self.bind_buffer(buffer_id, buffer_target)?;
        self.activate_attribute(&attribute_name, &opts)?;
        Ok(())
    }

    pub fn upload_buffer_to_attribute<B: BufferDataImpl>(
        &self,
        id: Id,
        data: B,
        attribute_name: &str,
        opts: &AttributeOptions,
    ) -> Result<(), Error> {
        self.upload_buffer(id, data)?;
        self.activate_attribute(&attribute_name, &opts)?;
        Ok(())
    }
}
