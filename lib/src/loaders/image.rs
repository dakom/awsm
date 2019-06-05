use futures::{Future};
use std::pin::{ Pin};
use futures::task::{Context, Poll};
use futures::channel::oneshot::{Sender, Receiver, channel};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use log::{info};
use crate::errors::{Error};
use crate::window::{get_window};

pub struct Image {
    pub url: String,
    pub img: Option<HtmlImageElement>,
    state: ImageState,
    closure_holders:Option<(Closure<dyn FnMut()>, Closure<dyn FnMut(JsValue)>)>,
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
//impl Future for Image {
    type Output = Result<HtmlImageElement, Error>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
    //fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match &mut self.state {
            ImageState::Empty => {
                let img = HtmlImageElement::new()?;
                let has_same_origin = same_origin(&self.url)?;
                if !has_same_origin {
                    img.set_cross_origin(Some(&"anonymous"));
                }

                img.set_src(&self.url);
                

                //success callback
                let waker = ctx.waker().clone();
                let (sender_success, receiver_success):(Sender<()>, Receiver<()>) = channel();
                let mut sender_success = Option::from(sender_success);
                let closure_success = Closure::wrap(Box::new(move || {
                    sender_success.take().unwrap().send(()).unwrap();
                    waker.wake_by_ref(); 
                }) as Box<dyn FnMut()>);
                
                img.set_onload(Some(closure_success.as_ref().unchecked_ref()));
                

                //error callback
                let waker = ctx.waker().clone();
                let (sender_err, receiver_err):(Sender<JsValue>, Receiver<JsValue>) = channel();
                let mut sender_err = Option::from(sender_err);
                let closure_err = Closure::wrap(Box::new(move |err| {
                    sender_err.take().unwrap().send(err).unwrap();
                    waker.wake_by_ref();
                }) as Box<dyn FnMut(JsValue)>);
                
                //self.closure_err = Some(closure_err);
                img.set_onerror(Some(closure_err.as_ref().unchecked_ref()));
                

                //Assign stuff to myself
                self.img = Some(img);
                self.state = ImageState::Loading {receiver_err, receiver_success};
                self.closure_holders = Some((closure_success, closure_err));

                //notify the task that we're now loading
                ctx.waker().wake_by_ref();


                Poll::Pending
            },

            ImageState::Loading {receiver_err, receiver_success} => {
                //if let Poll::Ready(value) = Receiver::poll(Pin::new(receiver_err), ctx) {

                let mut is_cancelled = false;

                let error_state = match receiver_err.try_recv() {
                    Ok(result) => result,
                    _ => {
                        is_cancelled = true;
                        None
                    }
                };

                let success_state = match receiver_success.try_recv() {
                    Ok(result) => result,
                    _ => {
                        is_cancelled = true;
                        None
                    }
                };


                let ret = {
                    if let Some(result) = error_state {
                        Poll::Ready(Err(result.into()))
                    } else if let Some(_) = success_state {
                        Poll::Ready(Ok(self.img.as_ref().unwrap().clone()))
                    } else {
                        info!("polling...");
                        if !is_cancelled {
                            //ctx.waker().wake_by_ref();
                        }
                        Poll::Pending
                    }
                };

                ret

            },
        }
    }
}

impl Image {
    pub fn new(url: String) -> Self {

        Self {
            url,
            img: None,
            state: ImageState::Empty,
            closure_holders: None,
        }
    }
}

pub fn fetch_image(url:String) -> impl Future<Output= Result<HtmlImageElement, Error>> { 
    Image::new(url)
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
