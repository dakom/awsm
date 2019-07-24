//! The tick module is based on the reference implementation at:
//! https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs

use crate::errors::Error;
use crate::window::get_window;
use log::info;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Window;

///Simple struct for time, deltatime, and elapsed time
#[derive(Copy, Clone, Debug)]
pub struct Timestamp {
    /// the current time
    pub time: f64,
    /// change in time since last tick
    pub delta: f64,
    /// total elapsed time since loop started
    pub elapsed: f64,
}

pub struct TimestampLoop {
    raf_loop: RafLoop,
}

impl TimestampLoop {
    /// similar to the top-level start_raf_loop() but instead of a callback with the current time
    /// it provides a Timestamp struct which contains commonly useful info
    pub fn start<F>(mut on_tick: F) -> Result<Self, Error>
    where
        F: (FnMut(Timestamp) -> ()) + 'static,
    {
        let mut last_time: Option<f64> = None;
        let mut first_time = 0f64;

        let raf_loop = RafLoop::start(move |time| {
            match last_time {
                Some(last_time) => {
                    on_tick(Timestamp {
                        time,
                        delta: time - last_time,
                        elapsed: time - first_time,
                    });
                }
                None => {
                    on_tick(Timestamp {
                        time,
                        delta: 0.0,
                        elapsed: 0.0,
                    });
                    first_time = time;
                }
            }
            last_time = Some(time);
        })?;

        Ok(Self { raf_loop })
    }
}

pub struct RafLoop {
    raf_id: Rc<Cell<Option<i32>>>,
}

impl Drop for RafLoop {
    fn drop(&mut self) {
        if let Some(id) = self.raf_id.get() {
            let window = get_window().unwrap();
            window.cancel_animation_frame(id).unwrap();
            self.raf_id.set(None);
        }
    }
}

impl RafLoop {
    /// Kick off a rAF loop. It will be cancelled when dropped
    pub fn start<F>(mut on_tick: F) -> Result<Self, Error>
    where
        F: (FnMut(f64) -> ()) + 'static,
    {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        //the main closure must be static - and so it needs to take in its deps via move
        //but keep_alive also exists in the struct / caller - so we're left with multiple owners

        let mut raf_id = Rc::new(Cell::new(None as Option<i32>));

        //this window is passed into the loop
        let window = get_window()?;
        {
            let raf_id = Rc::clone(&raf_id);
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time| {
                let id = raf_id.get();

                if id.is_some() {
                    raf_id.set(request_animation_frame(&window, f.borrow().as_ref().unwrap()).ok());
                    on_tick(time);
                }
            }) as Box<dyn FnMut(f64) -> ()>));
        }

        //this is just used to create the first invocation
        let window = get_window()?;
        raf_id.set(request_animation_frame(&window, g.borrow().as_ref().unwrap()).ok());

        Ok(Self { raf_id })
    }
}

fn request_animation_frame(
    window: &Window,
    f: &Closure<dyn FnMut(f64) -> ()>,
) -> Result<i32, Error> {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .map_err(|e| e.into())
}
