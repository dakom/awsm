use awsm_web::tick::{MainLoop, MainLoopOptions, RafLoop};
use awsm_renderer::Renderer;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub struct GameLoop {
    _raf_loop:RafLoop
}

impl GameLoop {
    pub fn new(renderer:Rc<RefCell<Renderer>>) -> Result<Self, JsValue> {
        // loop was ported from https://github.com/IceCreamYou/MainLoop.js#usage
        let begin = |_time, _delta| { };

        let update = {
            let renderer = Rc::clone(&renderer);
            move |delta| {
                let mut renderer = renderer.borrow_mut();
                renderer.animate(delta);
            }
        };

        let draw = {
            let renderer = Rc::clone(&renderer);
            move |interpolation| {
                let mut renderer = renderer.borrow_mut();

                renderer.clear();
                renderer.render(Some(interpolation));
            }
        };

        let end = |_fps, _abort| { };

        let raf_loop = RafLoop::start({
            let mut main_loop = MainLoop::new(MainLoopOptions::default(), begin, update, draw, end);
            move |ts| {
                main_loop.tick(ts);
            }
        })?;

        Ok(Self{
            _raf_loop: raf_loop
        })
    }
}