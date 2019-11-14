use std::fmt;
use wasm_bindgen::prelude::JsValue;
use awsm_web::errors::Error as AwsmWebError;

pub enum Error {
    Empty,
    String(String),
    Js(JsValue),
    Native(NativeError),
}

pub enum NativeError {
    Internal,
    GltfLoader,
    SceneMissing,
    AccessorView,
    Wip,
    NodeMissing(usize),
}

impl Error {
    pub fn to_js(self: &Self) -> JsValue {
        match self {
            Error::Empty => JsValue::null(),
            Error::String(s) => JsValue::from_str(&s[..]),
            Error::Js(jval) => jval.clone(),
            Error::Native(err) => JsValue::from_str(err.to_string().as_str()),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::Empty => write!(f, "empty error"),
            _ => write!(
                f,
                "{}",
                self.to_js()
                    .as_string()
                    .unwrap_or("unknown error".to_string())
            ),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::Empty => write!(f, "empty error"),
            _ => write!(
                f,
                "{}",
                self.to_js()
                    .as_string()
                    .unwrap_or("unknown error".to_string())
            ),
        }
    }
}

impl NativeError {
    pub fn default_str(self: &Self) -> &'static str {
        match self {
            NativeError::Internal => "internal error",
            NativeError::GltfLoader => "unable to load gltf",
            NativeError::SceneMissing=> "no such scene",
            NativeError::AccessorView => "non-sparse accessor must have a buffer view",
            NativeError::Wip => "Work In Progress",
            NativeError::NodeMissing(_) => "missing node",
        }
    }
    pub fn to_string(self: &Self) -> String {
        match self {
            NativeError::NodeMissing(index) => format!("missing node: {}", index),
            _ => self.default_str().to_string(),
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

impl From<AwsmWebError> for Error {
    fn from(err: AwsmWebError) -> Self {
        Error::Js(err.to_js())
    }
}

impl From<Error> for AwsmWebError {
    fn from(err: Error) -> Self {
        AwsmWebError::Js(err.to_js())
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

impl From<gltf::Error> for Error {
    fn from(err:gltf::Error) -> Self {
        Error::String(err.to_string())
    }
}

impl From<Error> for gltf::Error {
    fn from(err:Error) -> Self {
        //gltf:Error doesn't seem to implement a From for string
        //but io:Other isn't too confusing and actually fits the usual case
        gltf::Error::from(std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))
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
