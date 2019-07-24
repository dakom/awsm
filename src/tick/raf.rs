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
    raf_loop: RafLoop
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

        Ok(Self{raf_loop})
    }
}

pub struct RafLoop {
    keep_alive:Rc<Cell<bool>>
}

impl Drop for RafLoop {
    fn drop(&mut self) {
        self.keep_alive.set(false);
    }
}

impl RafLoop {

    /// Kick off a rAF loop. The returned function can be called to cancel it
    pub fn start<F>(mut on_tick: F) -> Result<Self, Error>
    where
        F: (FnMut(f64) -> ()) + 'static,
    {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        //the main closure must be static - and so it needs to take in its deps via move
        //but keep_alive also exists in cancel() - so we're left with multiple owners
        let keep_alive = Rc::new(Cell::new(true));

        let mut raf_id: Option<i32> = None;

        //this window is passed into the loop
        let window = get_window()?;
        {
            let keep_alive = Rc::clone(&keep_alive);
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time| {
                if !keep_alive.get() {
                    if let Some(id) = raf_id {
                        info!("clearing raf id: {}", id);

                        window.cancel_animation_frame(id).unwrap();
                    }
                    //stopping tick loop
                    f.borrow_mut().take();
                } else {
                    raf_id = request_animation_frame(&window, f.borrow().as_ref().unwrap()).ok();
                    on_tick(time);
                }
            }) as Box<dyn FnMut(f64) -> ()>));
        }

        //this is just used to create the first invocation
        let window = get_window()?;
        raf_id = request_animation_frame(&window, g.borrow().as_ref().unwrap()).ok();

        Ok(Self{
            keep_alive
        })
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
