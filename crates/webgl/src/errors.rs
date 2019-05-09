use wasm_bindgen::prelude::JsValue; 
use std::fmt;

pub enum Error {
    String(String),
    Js(JsValue),
    Native(NativeError)
}

pub enum NativeError {
    CanvasCreate,
    AttributeLocation,
    UniformLocation,
    MipsPowerOf2,
    NoExtension,
    MissingShaderProgram,
    NoCreateBuffer,
    NoExistingBuffer,
    NoCreateTexture,
    MissingTexture,
    MissingBuffer,
}

impl Error {
    pub fn to_js(self:&Self) -> JsValue {
        match self {
            Error::String(s) => JsValue::from_str(&s[..]),
            Error::Js(jval) => jval.clone(),
            Error::Native(err) => JsValue::from_str(err.default_str()),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_js().as_string().unwrap_or("unknown error".to_string()))
    }
}

impl NativeError {
    pub fn default_str (self:&Self) -> &'static str{
        match self {
            NativeError::CanvasCreate => "Couldn't create canvas",
            NativeError::AttributeLocation => "Couldn't get attribute location",
            NativeError::UniformLocation => "Couldn't get uniform location",
            NativeError::MipsPowerOf2 => "mipmapping requires that textures be power of 2",
            NativeError::NoExtension => "extension not found",
            NativeError::NoCreateBuffer => "couldn't create buffer",
            NativeError::NoExistingBuffer => "no existing buffer",
            NativeError::MissingShaderProgram => "No shader program activated",
            NativeError::NoCreateTexture => "unable to create texture",
            NativeError::MissingTexture => "couldn't get texture",
            NativeError::MissingBuffer => "couldn't get buffer",
        }
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        err.to_js()
    }
}

impl From<NativeError> for Error {
    fn from(err: NativeError) -> Self {
        Error::Native(err)
    }
}

impl From<JsValue> for Error {
    fn from(err: JsValue) -> Self {
        Error::Js(err)
    }
}

impl From<js_sys::Error> for Error {
    fn from(err: js_sys::Error) -> Self {
        Error::Js(JsValue::from(err))
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::String(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::String(String::from(err))
    }
}

/* TODO: this doesn't work, but maybe it could!
 * idea is to consolidate str and String into one impl
impl From<Borrow<str>> for Error 
{
    fn from(err: &str) -> Self {
        Error::String(String::from(err))
    }
}
*/
