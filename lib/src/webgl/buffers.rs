use web_sys::{WebGlBuffer};
use crate::errors::{Error, NativeError};
use super::{Id, WebGlCommon, WebGlRenderer, BufferTarget, BufferUsage};
use web_sys::{WebGlRenderingContext,WebGl2RenderingContext};
use std::marker::PhantomData;

/*
 * The direct uniform uploads are written as traits on this newtype wrapper
 * in order to allow working either f32 or u8
 */

//See: https://users.rust-lang.org/t/different-impls-for-types-of-slices-and-arrays
//
//

pub trait PartialWebGlBuffer {
    fn awsm_upload_buffer_vf32<T: AsRef<[f32]>>(&self, target:BufferTarget, usage: BufferUsage, data:T);
    fn awsm_upload_buffer_vf32_sub<T: AsRef<[f32]>>(&self, target:BufferTarget, dest_byte_offset:u32, src_offset:u32, length: u32, data:T) -> Result<(), Error>;
    fn awsm_upload_buffer_vu8<T: AsRef<[u8]>>(&self, target:BufferTarget, usage: BufferUsage, data:T);
    fn awsm_upload_buffer_vu8_sub<T: AsRef<[u8]>>(&self, target:BufferTarget, dest_byte_offset:u32, src_offset:u32, length: u32, data:T) -> Result<(), Error>;


    fn awsm_bind_buffer(&self, target:BufferTarget, buffer:&WebGlBuffer);
    fn awsm_release_buffer(&self, target:BufferTarget);
    fn awsm_create_buffer(&self) -> Result<WebGlBuffer, Error>;
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlBuffer for $type {
            fn awsm_upload_buffer_vf32<T: AsRef<[f32]>>(&self, target:BufferTarget, usage: BufferUsage, data:T) {
                unsafe {
                    let typed_array = js_sys::Float32Array::view(data.as_ref());
                    self.buffer_data_with_array_buffer_view(target as u32, &typed_array, usage as u32); 
                }
            }
            
            fn awsm_upload_buffer_vu8<T: AsRef<[u8]>>(&self, target:BufferTarget, usage: BufferUsage, data:T) {
                let values = data.as_ref();
                self.buffer_data_with_u8_array(target as u32, &values, usage as u32); 
            }

            fn awsm_bind_buffer(&self, target:BufferTarget, buffer:&WebGlBuffer) {
                self.bind_buffer(target as u32, Some(buffer)); 
            }

            fn awsm_release_buffer(&self, target:BufferTarget) {
                self.bind_buffer(target as u32, None); 
            }

            fn awsm_create_buffer(&self) -> Result<WebGlBuffer, Error> {
                self.create_buffer().ok_or(Error::from(NativeError::NoCreateBuffer))
            }

            $($defs)*
        })+
    };
}

impl_context!{
    WebGlRenderingContext{
        fn awsm_upload_buffer_vf32_sub<T: AsRef<[f32]>>(&self, target:BufferTarget, dest_byte_offset:u32, src_offset:u32, length: u32, data:T) -> Result<(), Error> {
            if src_offset != 0 {
                Err(Error::from(NativeError::WebGlBufferSourceOneNonZero))
            } else {
                unsafe {
                    let typed_array = js_sys::Float32Array::view(data.as_ref());
                    self.buffer_sub_data_with_f64_and_array_buffer_view(target as u32, dest_byte_offset as f64, &typed_array); 
                }
                Ok(())
            }
        }


        fn awsm_upload_buffer_vu8_sub<T: AsRef<[u8]>>(&self, target:BufferTarget, dest_byte_offset:u32, src_offset:u32, length: u32, data:T) -> Result<(), Error> {
            if src_offset != 0 {
                Err(Error::from(NativeError::WebGlBufferSourceOneNonZero))
            } else {
                self.buffer_sub_data_with_f64_and_u8_array(target as u32, dest_byte_offset as f64, &data.as_ref()); 
                Ok(())
            }
        }
    }
    WebGl2RenderingContext{
    
        fn awsm_upload_buffer_vf32_sub<T: AsRef<[f32]>>(&self, target:BufferTarget, dest_byte_offset:u32, src_offset:u32, length: u32, data:T) -> Result<(), Error> {
            unsafe {
                let typed_array = js_sys::Float32Array::view(data.as_ref());
                self.buffer_sub_data_with_f64_and_array_buffer_view_and_src_offset_and_length(
                    target as u32, 
                    dest_byte_offset as f64,
                    &typed_array,
                    src_offset,
                    length
                );
            }
            Ok(())
        }
        fn awsm_upload_buffer_vu8_sub<T: AsRef<[u8]>>(&self, target:BufferTarget, dest_byte_offset:u32, src_offset:u32, length: u32, data:T) -> Result<(), Error> {
            self.buffer_sub_data_with_f64_and_u8_array_and_src_offset_and_length(
                target as u32, 
                dest_byte_offset as f64,
                &data.as_ref(),
                src_offset,
                length
            );
            Ok(())
        }
    }
}

pub struct BufferData<T, U>{
    values: T, 
    target: BufferTarget, 
    usage: BufferUsage, 
    phantom: PhantomData<U>
}

impl<T: AsRef<[U]>, U> BufferData<T, U> {
    pub fn new(values: T, target:BufferTarget, usage:BufferUsage) -> Self {
        Self {
            values, 
            target, 
            usage, 
            phantom: PhantomData
        }
    }
}

pub trait BufferDataImpl {
    fn upload_buffer<G: PartialWebGlBuffer>(&self, gl:&G);
    fn upload_buffer_sub<G: PartialWebGlBuffer>(&self, gl:&G, dest_byte_offset:u32, src_offset:u32, length: u32) -> Result<(), Error>;
    fn get_target(&self) -> BufferTarget;
    fn get_usage(&self) -> BufferUsage;
}

//see example: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/webgl/src/lib.rs#L42
impl <T: AsRef<[f32]>> BufferDataImpl for BufferData<T, f32> {
    fn upload_buffer<G: PartialWebGlBuffer>(&self, gl:&G) {
        gl.awsm_upload_buffer_vf32(self.target, self.usage, &self.values)
    }

    fn upload_buffer_sub<G: PartialWebGlBuffer>(&self, gl:&G, dest_byte_offset:u32, src_offset:u32, length:u32) -> Result<(), Error> {
        gl.awsm_upload_buffer_vf32_sub(self.target, dest_byte_offset, src_offset, length, &self.values)
    }
    fn get_target(&self) -> BufferTarget {
        self.target
    }
    fn get_usage(&self) -> BufferUsage {
        self.usage
    }
}


impl <T: AsRef<[u8]>> BufferDataImpl for BufferData<T, u8> {

    fn upload_buffer<G: PartialWebGlBuffer>(&self, gl:&G) {
        gl.awsm_upload_buffer_vu8(self.target, self.usage, &self.values)
    }

    fn upload_buffer_sub<G: PartialWebGlBuffer>(&self, gl:&G, dest_byte_offset:u32, src_offset:u32, length:u32) -> Result<(), Error> {
        gl.awsm_upload_buffer_vu8_sub(self.target, dest_byte_offset, src_offset, length, &self.values)
    }

    fn get_target(&self) -> BufferTarget {
        self.target
    }
    fn get_usage(&self) -> BufferUsage {
        self.usage
    }
}

//renderer impl

impl <T: WebGlCommon> WebGlRenderer<T> {
    pub fn create_buffer(&mut self) -> Result<Id, Error> {
        let buffer = self.gl.awsm_create_buffer()?;
        let id = self.buffer_lookup.insert(buffer);

        Ok(id)
    }


    //only pub within the module - used elsewhere like attributes
    pub(super) fn _bind_buffer_nocheck(&self, buffer_id:Id, target: BufferTarget) -> Result<(), Error> {

        self.current_buffer_id.set(Some(buffer_id));
        self.current_buffer_target.set(Some(target));
        self.current_buffer_index.set(None);

        let buffer = self.buffer_lookup.get(buffer_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;
        self.gl.awsm_bind_buffer(target, &buffer);

        Ok(())
    }


    pub fn bind_buffer(&self, buffer_id:Id, target: BufferTarget) -> Result<(), Error> {

        if Some(buffer_id) != self.current_buffer_id.get() || Some(target) != self.current_buffer_target.get() {
            self._bind_buffer_nocheck(buffer_id, target)
        } else {
            Ok(())
        }
    }

    pub fn release_buffer(&self, target:BufferTarget) {
        self.current_buffer_id.set(None);
        self.current_buffer_target.set(Some(target));
        self.current_buffer_index.set(None);

        self.gl.awsm_release_buffer(target);
    }

    pub fn upload_buffer<B: BufferDataImpl>(&self, id:Id, data:B) -> Result<(), Error> {
        self.bind_buffer(id, data.get_target())?;
        data.upload_buffer(&self.gl);
        Ok(())
    }

    
    //note - dest_byte_offset are the BYTE offset (e.g. 4 for floats)
    //src_offset and length are element amounts
    //example: 4,1,1 will update the second float in the buffer (i.e. 4 bytes in)
    //from the second float in the source (i.e. 1 element in) and just be one float (i.e. 1
    //element)
    //
    //WebGl1 only supports a src_offset of 0
    pub fn upload_buffer_sub<B: BufferDataImpl>(&self, id:Id, dest_byte_offset:u32, src_offset:u32, length:u32, data:B) -> Result<(), Error> {
        self.bind_buffer(id, data.get_target())?;
        data.upload_buffer_sub(&self.gl, dest_byte_offset, src_offset, length)
    }
}


impl WebGlRenderer<WebGl2RenderingContext> {
    pub(super) fn _bind_buffer_base_nocheck(&self, buffer_id:Id, index: u32, target: BufferTarget) -> Result<(), Error> {

        self.current_buffer_id.set(Some(buffer_id));
        self.current_buffer_target.set(Some(target));
        self.current_buffer_index.set(Some(index));
        
        let buffer = self.buffer_lookup.get(buffer_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;
        self.gl.bind_buffer_base(target as u32, index, Some(buffer)); 

        Ok(())
    }

    pub fn bind_buffer_base(&self, buffer_id:Id, index: u32, target: BufferTarget) -> Result<(), Error> {

        if Some(buffer_id) != self.current_buffer_id.get() 
            || Some(target) != self.current_buffer_target.get() 
            || Some(index) != self.current_buffer_index.get() {
                self._bind_buffer_base_nocheck(buffer_id, index, target)
        } else {
            Ok(())
        }
    }
}
