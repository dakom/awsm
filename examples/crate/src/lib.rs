mod router;
mod menu;
mod scenes;

use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;
use log::{Level, warn, info};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging only during debug builds 
cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use console_log;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

// enable panic hook only during debug builds
cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        fn init_panic() {
            console_error_panic_hook::set_once();
        }
    } else {
        fn init_panic() {}
    }
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    init_panic();
    init_log();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    info!("it works!");

    router::start_router(window, document)
}

