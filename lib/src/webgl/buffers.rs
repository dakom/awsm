use web_sys::{WebGlBuffer};
use js_sys::{WebAssembly};
use wasm_bindgen::JsCast;
use crate::errors::{Error, NativeError};
use super::{WebGlContext, Id, AttributeOptions, WebGlRenderer, BufferTarget, BufferUsage};
use std::marker::PhantomData;

/*
 * The direct uniform uploads are written as traits on this newtype wrapper
 * in order to allow working either f32 or u8
 */

//See: https://users.rust-lang.org/t/different-impls-for-types-of-slices-and-arrays
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
    fn upload_buffer(&self, gl:&WebGlContext) -> Result<(), Error>;
    fn get_target(&self) -> BufferTarget;
    fn get_usage(&self) -> BufferUsage;
}


//TODO - impls with buffer_sub_data
impl <T: AsRef<[f32]>> BufferDataImpl for BufferData<T, f32> {
    fn upload_buffer(&self, gl:&WebGlContext) -> Result<(), Error> {
        wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .map(|m:WebAssembly::Memory| {
                let values = self.values.as_ref();

                let wasm_buffer = m.buffer();
                let ptr_loc = values.as_ptr() as u32 / 4;

                let float32 = js_sys::Float32Array::new(&wasm_buffer)
                                .subarray(ptr_loc, ptr_loc + values.len() as u32);
        
                //Note - WebGL2 can do less GC hits by pointing at same memory with different start/end
                gl.buffer_data_with_array_buffer_view(self.target as u32, &float32, self.usage as u32); 
                
            })
            .map_err(|err| Error::from(err))
    }

    fn get_target(&self) -> BufferTarget {
        self.target
    }
    fn get_usage(&self) -> BufferUsage {
        self.usage
    }
}


impl <T: AsRef<[u8]>> BufferDataImpl for BufferData<T, u8> {
    fn upload_buffer(&self, gl:&WebGlContext) -> Result<(), Error> {
        let values = self.values.as_ref();
        gl.buffer_data_with_u8_array(self.target as u32, &values, self.usage as u32); 
        Ok(())
    }

    fn get_target(&self) -> BufferTarget {
        self.target
    }
    fn get_usage(&self) -> BufferUsage {
        self.usage
    }
}

pub fn bind_buffer_direct(gl:&WebGlContext, target:BufferTarget, buffer:&WebGlBuffer) {
    gl.bind_buffer(target as u32, Some(buffer)); 
}

#[cfg(feature = "webgl_2")]
pub fn bind_buffer_base_direct(gl:&WebGlContext, target:BufferTarget, index:u32, buffer:&WebGlBuffer) {
    gl.bind_buffer_base(target as u32, index, Some(buffer)); 
}

impl WebGlRenderer {
    pub fn create_buffer(&mut self) -> Result<Id, Error> {
        let buffer = self.gl.create_buffer()
            .ok_or(Error::from(NativeError::NoCreateBuffer))?;

        let id = self.buffer_lookup.insert(buffer);

        Ok(id)
    }


    //only pub within the module - used elsewhere like attributes
    pub(super) fn _bind_buffer_nocheck(&self, buffer_id:Id, target: BufferTarget) -> Result<(), Error> {

        self.current_buffer_id.set(Some(buffer_id));
        self.current_buffer_target.set(Some(target));
        self.current_buffer_index.set(None);

        let buffer = self.get_current_buffer()?; 
        bind_buffer_direct(&self.gl, target, &buffer);

        Ok(())
    }


    pub fn bind_buffer(&self, buffer_id:Id, target: BufferTarget) -> Result<(), Error> {

        if Some(buffer_id) != self.current_buffer_id.get() || Some(target) != self.current_buffer_target.get() {
            self._bind_buffer_nocheck(buffer_id, target)
        } else {
            Ok(())
        }
    }

    #[cfg(feature = "webgl_2")]
    pub(super) fn _bind_buffer_base_nocheck(&self, buffer_id:Id, index: u32, target: BufferTarget) -> Result<(), Error> {

        self.current_buffer_id.set(Some(buffer_id));
        self.current_buffer_target.set(Some(target));
        self.current_buffer_index.set(Some(index));
        let buffer = self.get_current_buffer()?; 
        bind_buffer_base_direct(&self.gl, target, index, &buffer);

        Ok(())
    }

    #[cfg(feature = "webgl_2")]
    pub fn bind_buffer_base(&self, buffer_id:Id, index: u32, target: BufferTarget) -> Result<(), Error> {

        if Some(buffer_id) != self.current_buffer_id.get() 
            || Some(target) != self.current_buffer_target.get() 
            || Some(index) != self.current_buffer_index.get() {
                self._bind_buffer_base_nocheck(buffer_id, index, target)
        } else {
            Ok(())
        }
    }
    fn get_current_buffer(&self) -> Result<&WebGlBuffer, Error> {
        let buffer_id = self.current_buffer_id.get().ok_or(Error::from(NativeError::MissingBuffer))?;
        self.buffer_lookup.get(buffer_id).ok_or(Error::from(NativeError::MissingShaderProgram))
    }

    pub fn upload_buffer<T: BufferDataImpl>(&self, id:Id, data:T) -> Result<(), Error> {
        self.bind_buffer(id, data.get_target())?;
        data.upload_buffer(&self.gl)
    }
}
