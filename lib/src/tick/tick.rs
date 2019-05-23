use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub struct Timestamp {
    pub time: f64,
    pub delta: f64,
    pub elapsed: f64,
}


//Kick off rAF loop
//keep_alive is a boolean to stop the loop
//It will wait one tick at the beginning in order to have sensible delta_time and elapsed_time values
pub fn start_loop<F>(keep_alive: Rc<Cell<bool>>, mut on_tick:F) -> Result<(), JsValue> 
where F: (FnMut(Timestamp) -> ()) + 'static
{

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut last_time:Option<f64> = None;
    let mut first_time = 0f64;

    {
        //see: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time| {
           
            if !keep_alive.get() {
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

                request_animation_frame(f.borrow().as_ref().unwrap())
                    .ok()
                    .unwrap();
            }
        }) as Box<FnMut(f64)-> ()>));
    }

    request_animation_frame(g.borrow().as_ref().unwrap())?;
    
    Ok(())
}

fn request_animation_frame(f: &Closure<FnMut(f64) -> ()>) -> Result<i32, JsValue> {
    let window = web_sys::window().expect("couldn't get window!");
    window.request_animation_frame(f.as_ref().unchecked_ref())
}
