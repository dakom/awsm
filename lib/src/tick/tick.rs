use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
use web_sys::{console, Window};

#[derive(Copy, Clone, Debug)]
pub struct Timestamp {
    pub time: f64,
    pub delta: f64,
    pub elapsed: f64,
}

//Kick off rAF loop
//keep_alive is a boolean to stop the loop
//It will wait one tick at the beginning in order to have sensible delta_time and elapsed_time values
pub fn start_loop<F>(mut on_tick:F) -> Result<impl (FnOnce() -> ()), JsValue> 
where F: (FnMut(Timestamp) -> ()) + 'static
{

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    //the main closure must be static - and so it needs to take in its deps via move
    //but keep_alive also exists in cancel() - so we're left with multiple owners
    let keep_alive = Rc::new(Cell::new(true));
    
    let mut last_time:Option<f64> = None;
    let mut first_time = 0f64;

    let mut raf_id:Option<i32> = None;

    //this window is passed into the loop
    let window = web_sys::window().expect("couldn't get window!");
    {
        let keep_alive = Rc::clone(&keep_alive);
        //see: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time| {
           
            if !keep_alive.get() {
                if let Some(id) = raf_id {
                    console::log_1(&JsValue::from_str(format!("clearing raf id {}", id).as_ref()));

                    window.cancel_animation_frame(id).unwrap();
                }
                //stopping tick loop
                f.borrow_mut().take();
            } else {
                match last_time {
                    Some(last_time) => {
                        on_tick(Timestamp{
                            time,
                            delta: time - last_time,
                            elapsed: time - first_time
                        });
                    }
                    None => {
                        first_time = time;
                    }
                }
                last_time = Some(time);

                raf_id = request_animation_frame(&window, f.borrow().as_ref().unwrap()).ok();
            }
        }) as Box<FnMut(f64)-> ()>));
    }

    //this is just used to create the first invocation
    let window = web_sys::window().expect("couldn't get window!");
    request_animation_frame(&window, g.borrow().as_ref().unwrap())?;
   
    let cancel = move || keep_alive.set(false);

    Ok(cancel)
}

fn request_animation_frame(window:&Window, f: &Closure<FnMut(f64) -> ()>) -> Result<i32, JsValue> {
    window.request_animation_frame(f.as_ref().unchecked_ref())
}
