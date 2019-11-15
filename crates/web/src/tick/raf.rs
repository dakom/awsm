//! The tick module is based on the reference implementation at:
//! https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs

use crate::errors::Error;
use crate::global::{get_global_self, GlobalSelf, GlobalSelfPreference};
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Window;
use web_sys::WorkerGlobalScope;

pub struct RafLoop {
    raf_state: RafState,
    raf_id: Rc<Cell<Option<i32>>>,
}

impl Drop for RafLoop {
    fn drop(&mut self) {
        if let Some(id) = self.raf_id.get() {
            match &self.raf_state {
                RafState::Window(state) => {
                    state.borrow().window.cancel_animation_frame(id).unwrap();
                },
                RafState::Worker(state) => {
                    state.borrow().worker.clear_timeout_with_handle(id);
                },
            }
            self.raf_id.set(None);
        }

        #[cfg(feature = "debug_log")]
        log::info!("Raf Loop Dropped (but not whatever was passed into the Closure!");
    }
}

enum RafState {
    Window(Rc<RefCell<WindowRafState>>),
    Worker(Rc<RefCell<WorkerRafState>>)
}

struct WindowRafState {
    window: Window,
}
struct WorkerRafState {
    worker: WorkerGlobalScope,
    timestep: f64,
    last_timestamp: f64
}

impl RafLoop {
    /// Kick off a rAF loop. It will be cancelled when dropped
    pub fn start_with_fallback_timestep<F>(global_self_preference: Option<GlobalSelfPreference>, fallback_timestep: f64, mut on_tick: F) -> Result<Self, Error>
    where
        F: (FnMut(f64) -> ()) + 'static,
    {
        //Use a window or worker with fallback method of simulated timestep for requestAnimationFrame
        let global_self = get_global_self (global_self_preference)?; 
        let raf_id = Rc::new(Cell::new(None as Option<i32>));

        match global_self {
            GlobalSelf::Window(window) => {

                let f = Rc::new(RefCell::new(None));
                let g = f.clone();

                let raf_state = WindowRafState{window};
                let raf_state = Rc::new(RefCell::new(raf_state));

                //this window is passed into the loop
                {
                    let raf_state = Rc::clone(&raf_state);
                    let raf_id = Rc::clone(&raf_id);
                    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time| {
                        let id = raf_id.get();
                        if id.is_some() {
                            {
                                let state = raf_state.borrow_mut();
                                raf_id.set(request_animation_frame(&state.window, f.borrow().as_ref().unwrap()));
                            }
                            on_tick(time);
                        } 
                    }) as Box<dyn FnMut(f64) -> ()>));
                }

                //this is just used to create the first invocation
                let state = raf_state.borrow_mut();
                raf_id.set(request_animation_frame(&state.window, g.borrow().as_ref().unwrap()));
                Ok(Self { raf_state: RafState::Window(Rc::clone(&raf_state)), raf_id})
            },
            GlobalSelf::Worker(worker) => {
                let raf_state = WorkerRafState{ worker, timestep: fallback_timestep, last_timestamp: get_now_for_worker()};
                let raf_state = Rc::new(RefCell::new(raf_state));
                
                let f = Rc::new(RefCell::new(None));
                let g = f.clone();


                //this window is passed into the loop
                {
                    let raf_state = Rc::clone(&raf_state);
                    let raf_id = Rc::clone(&raf_id);
                    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                        let id = raf_id.get();
                        if id.is_some() {
                            let time = 
                            {
                                let mut state = raf_state.borrow_mut();
                                let now = get_now_for_worker(); 
                                let timeout = 0.0f64.max(state.timestep - (now - state.last_timestamp));
                                state.last_timestamp = now + timeout;
                                raf_id.set(pseudo_animation_frame(&state.worker, timeout, f.borrow().as_ref().unwrap()));
                                now + timeout
                            };
                            on_tick(time);
                        }
                    }) as Box<dyn FnMut() -> ()>));
                }

                //this is just used to create the first invocation
                let mut state = raf_state.borrow_mut();
                let now = get_now_for_worker(); 
                let timeout = 0.0f64.max(state.timestep - (now - state.last_timestamp));
                state.last_timestamp = now + timeout;
                raf_id.set(pseudo_animation_frame(&state.worker, timeout, g.borrow().as_ref().unwrap()));
                Ok(Self { raf_state: RafState::Worker(Rc::clone(&raf_state)), raf_id})
            }
        }

    }


    pub fn start<F>(on_tick: F) -> Result<Self, Error>
    where
        F: (FnMut(f64) -> ()) + 'static,
    {
        Self::start_with_fallback_timestep(None, 1000.0 / 16.0, on_tick)
    }
}

fn request_animation_frame(window:&Window, f: &Closure<dyn FnMut(f64) -> ()>) -> Option<i32> {
    window.request_animation_frame(f.as_ref().unchecked_ref()).ok()
}

fn pseudo_animation_frame(worker:&WorkerGlobalScope, timeout: f64, f: &Closure<dyn FnMut() -> ()>) -> Option<i32> {
    worker.set_timeout_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), timeout as i32).ok()
}

fn get_now_for_worker() -> f64 {
    //TODO - performance.now? Should be available in Chrome and Firefox at least...
    js_sys::Date::now() 
}