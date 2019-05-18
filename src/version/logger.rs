use web_sys::{console};
use wasm_bindgen::{JsValue};
use super::info::*;

pub fn log_version() {
    console::log_1(&JsValue::from_str(version_info()));
}