mod rust;

extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate rand;

use crate::rust::scenes::scene::{Scene};
use crate::rust::scenes::basic::quad::quad_scene::*;
use crate::rust::scenes::basic::quad_texture::quad_texture_scene::*;
use crate::rust::scenes::basic::instancing::instancing_scene::*;
use crate::rust::dom_handlers::*;
use wasm_bindgen::prelude::*;
use awsm_webgl::renderer::*; 
use awsm_webgl::errors::*; 
use std::rc::Rc;
use std::cell::RefCell;
use futures::future::{Future, result};
use wasm_bindgen_futures::{future_to_promise};

#[wasm_bindgen]
pub extern "C" fn run(
    canvas: web_sys::HtmlCanvasElement, 
    scene_name: String, 
) -> js_sys::Promise {

    let _this = &JsValue::NULL;

    //console::log_1(&JsValue::from_str(format!("Loading {}", scene_name).as_str()));


    future_to_promise(
        //Yeah this is getting nasty...
        //Should look MUCH nicer if we can use async/await
        result(WebGlRenderer::new(canvas))
            .and_then(move |webgl_renderer| {
                let webgl_renderer = Rc::new(RefCell::new(webgl_renderer));
                get_scene(scene_name.as_str(), Rc::clone(&webgl_renderer))
                    .map(|scene| Rc::new(RefCell::new(scene)))
                    .and_then(move |scene| {

                        let keep_alive = Rc::new(RefCell::new(true));

                        result(
                            start_ticker(Rc::clone(&keep_alive), Rc::clone(&scene))
                                .and_then(|_| {
                                    start_resize(Rc::clone(&webgl_renderer), Rc::clone(&scene))
                                })
                                .map(|mut cleanup_resizer| {
                                    let cleanup_cb = Closure::wrap(Box::new(move || {
                                        //console::log_1(&JsValue::from_str("cleanup"));
                                        cleanup_resizer();
                                        *keep_alive.borrow_mut() = false;
                                    }) as Box<FnMut()>);

                                    //is there a nicer way to do this??
                                    let js_cleanup_cb = JsValue::from(cleanup_cb.as_ref());
                                    Closure::forget(cleanup_cb);

                                    js_cleanup_cb 

                                })
                        )
                    })
            })
            .map_err(|err| err.to_js())
            //.map(|f| { JsValue::from_str("it works!") })
    )
}

//should work... see: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=adfb0e3366e47fa59a4942a48376f685
fn get_scene(scene_name:&str, webgl_renderer:Rc<RefCell<WebGlRenderer<'static>>>) -> Box<dyn Future<Item = Box<dyn Scene + 'static>, Error = Error>>{

    match scene_name {
        "quad" => Box::new(QuadScene::new(webgl_renderer).map(|scene| scene as Box<Scene + 'static>)),
        "quad_texture" => Box::new(QuadTextureScene::new(webgl_renderer).map(|scene| scene as Box<Scene + 'static>)),
        "instancing" => Box::new(InstancingScene::new(webgl_renderer).map(|scene| scene as Box<Scene + 'static>)),
        _ => Box::new(futures::future::err(Error::from("unknown scene!")))
    }
}
