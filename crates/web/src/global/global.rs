use crate::errors::{Error, NativeError};
use wasm_bindgen::JsCast;
use web_sys::Window;
use web_sys::WorkerGlobalScope;

pub enum WindowOrWorker {
    Window(Window),
    Worker(WorkerGlobalScope)
}

pub fn get_window_or_worker() -> Result<WindowOrWorker, Error> {
    if let Ok(window) = js_sys::global().dyn_into::<Window>() {
        return Ok(WindowOrWorker::Window(window));
    } else if let Ok(worker) = js_sys::global().dyn_into::<WorkerGlobalScope>() {
        return Ok(WindowOrWorker::Worker(worker));
    }

    Err(Error::from(NativeError::WindowOrWorker))
}