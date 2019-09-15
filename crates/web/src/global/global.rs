use crate::errors::{Error, NativeError};
use wasm_bindgen::JsCast;
use web_sys::Window;
use web_sys::WorkerGlobalScope;

pub enum GlobalSelf {
    Window(Window),
    Worker(WorkerGlobalScope)
}

#[derive(Copy,Clone)]
pub enum GlobalSelfPreference {
    Window,
    Worker
}

/**
 * returns the global self, trying the preference first and if that fails, the other
 * default preference is window, which means if that fails it'll try worker
 */
pub fn get_global_self (preference:Option<GlobalSelfPreference>) -> Result<GlobalSelf, Error> {

    let preference = preference.unwrap_or(GlobalSelfPreference::Window);

    match preference {
        GlobalSelfPreference::Window => {
            match get_window() {
                Some(res) => Ok(res),
                None => {
                    match get_worker() {
                        Some(res) => Ok(res),
                        None => Err(Error::from(NativeError::GlobalSelf))
                    }
                }
            }
        },

        GlobalSelfPreference::Worker => {
            match get_worker() {
                Some(res) => Ok(res),
                None => {
                    match get_window() {
                        Some(res) => Ok(res),
                        None => Err(Error::from(NativeError::GlobalSelf))
                    }
                }
            }
        },
    }
    
}


fn get_window() -> Option<GlobalSelf> {
    js_sys::global().dyn_into::<Window>()
        .ok()
        .map(GlobalSelf::Window)
}


fn get_worker() -> Option<GlobalSelf> {
    js_sys::global().dyn_into::<WorkerGlobalScope>()
        .ok()
        .map(GlobalSelf::Worker)
}