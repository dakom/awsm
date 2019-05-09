extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use futures::{Future, Async, Poll};
use futures::sync::oneshot::{Sender, Receiver, channel};
use futures::task::current;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

struct Image {
    url: String,
    img: Option<HtmlImageElement>,
    state: ImageState,
    closureHolders:Option<(Closure<FnMut()>, Closure<FnMut(JsValue)>)>,
}

enum ImageState {
    Empty,
    Loading {
        receiver_err: Receiver<JsValue>,
        receiver_success: Receiver<()>,
    },
}

//See: https://github.com/rustwasm/wasm-bindgen/issues/1126
//
impl Future for Image {
    type Item = HtmlImageElement;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match &mut self.state {
            ImageState::Empty => {
                let img = HtmlImageElement::new()?;
                let url = self.url.as_str();
                let has_same_origin = same_origin(url)?;
                if !has_same_origin {
                    img.set_cross_origin(Some(&"anonymous"));
                }

                img.set_src(url);
                

                //success callback
                let task = current();
                let (sender_success, receiver_success):(Sender<()>, Receiver<()>) = channel();
                let mut sender_success = Option::from(sender_success);
                let closure_success = Closure::wrap(Box::new(move || {
                    sender_success.take().unwrap().send(());
                    task.notify();
                }) as Box<FnMut()>);
                
                img.set_onload(Some(closure_success.as_ref().unchecked_ref()));
                

                //error callback
                let (sender_err, receiver_err):(Sender<JsValue>, Receiver<JsValue>) = channel();
                let mut sender_err = Option::from(sender_err);
                let task = current();
                let closure_err = Closure::wrap(Box::new(move |err| {
                    sender_err.take().unwrap().send(err);
                    task.notify();
                }) as Box<FnMut(JsValue)>);
                
                //self.closure_err = Some(closure_err);
                img.set_onerror(Some(closure_err.as_ref().unchecked_ref()));
                

                //Assign stuff to myself
                self.img = Some(img);
                self.state = ImageState::Loading {receiver_err, receiver_success};
                self.closureHolders = Some((closure_success, closure_err));

                //notify the task that we're now loading
                let task = current();
                task.notify();

                Ok(Async::NotReady)
            },

            ImageState::Loading {receiver_err, receiver_success} => {
                let mut ret = Ok(Async::NotReady);

                if let Ok(value) = receiver_err.poll() {
                    if let Async::Ready(err) = value {
                        ret = Err(err);
                    }                     
                }

                if let Ok(value) = receiver_success.poll() {
                    if let Async::Ready(_) = value {
                        ret = Ok(Async::Ready(self.img.as_ref().unwrap().clone()));
                    }                     
                }
                
                ret

            },
        }
    }
}

impl Image {
    fn new(url: String) -> Self {

        Self {
            url,
            img: None,
            state: ImageState::Empty,
            closureHolders: None,
        }
    }
}

pub fn fetch_image(url:String) -> impl Future<Item = HtmlImageElement, Error = JsValue> { 
    Image::new(url)

    /*
    let img = HtmlImageElement::new()?;

    let has_same_origin = same_origin(url)?;

    if !has_same_origin {
        img.set_cross_origin(Some(&"anonymous"));
    }

    let cb_onload = |evt:JsValue| {
        //evt is a JsValue... I think... we want to get evt.target
        console::log_1(&evt);
    };

    let cb_onload = Closure::wrap(Box::new(cb_onload) as Box<Fn(JsValue) -> ()>);

    img.set_onload(Some(cb_onload.as_ref().unchecked_ref()));


    let cb_onerror = |evt:JsValue| {
        //evt is a JsValue... I think... we want to get evt.target
        console::error_1(&evt);
    };

    let cb_onerror = Closure::wrap(Box::new(cb_onerror) as Box<Fn(JsValue) -> ()>);

    img.set_onerror(Some(cb_onerror.as_ref().unchecked_ref()));

    //TODO - manage this better... should return in a future somehow?
    cb_onload.forget();
    cb_onerror.forget();

    img.set_src(url);
    Ok(img)
    */
}


pub fn same_origin(url:&str) -> Result<bool, JsValue> {
    //FOLLOWUP: https://github.com/rustwasm/wasm-bindgen/issues/1150
    if url.starts_with("http://") || url.starts_with("https://") {
        let location_origin = get_window()?.location().origin()?; 
        let url_origin = Url::new(url)?.origin();
        Ok(url_origin == location_origin)
    } else {
        Ok(true)
    }
}

fn get_window () -> Result<web_sys::Window, JsValue> {
    web_sys::window().ok_or(JsValue::from_str("couldn't get window"))
}
