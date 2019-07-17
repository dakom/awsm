use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, Element, Document, MouseEvent};
use js_sys::{Object};
use crate::errors::{Error, NativeError};
use gloo_events::{EventListener};
use log::{info};
use std::cell::{Cell, RefCell};
use std::rc::{Rc};

pub fn listen_pointer_lock<F, G, H>(trigger:&Element, target:&Element, document:&Document, on_start: F, on_move:G, on_end: H) -> impl FnOnce() -> ()
    where 
    F: Fn(&Element) + 'static,
    G: Fn(i32, i32) + 'static, 
    H: Fn() + 'static
    {
        let is_locked = Rc::new(Cell::new(false));

        let request_lock = {
            let target = target.clone();
            let is_locked = Rc::clone(&is_locked);
            move |_:&_| {
                if !is_locked.get() {
                    target.request_pointer_lock();
                    is_locked.set(true);
                }
            }
        };

        let on_pointer_move = Rc::new(move |evt:&Event| {
            let evt:MouseEvent = JsValue::from(evt).into(); 
            let (x,y) = (evt.movement_x(), evt.movement_y());

            on_move(x,y);
        });


        let pointer_lock_change = {
            let target = target.clone();
            let document = document.clone();
            let is_locked = Rc::clone(&is_locked);

            let mut listener = None;

            Rc::new(RefCell::new(move |initial_evt:&Event| {

                let lock_enabled = match document.pointer_lock_element() {
                    None => false,
                    Some(element) => elements_are_equal(&element, &target) 
                };
                is_locked.set(lock_enabled);

                if lock_enabled {
                    on_start(&target);

                    listener = Some(EventListener::new(&document, "mousemove", {
                        let f = on_pointer_move.clone();
                        move |e| f(e)
                    }));
                } else {
                    on_end();
                    listener.take();
                }
            }))
        };

        let mut event_listeners = Some((
            EventListener::new(&document, "pointerlockchange",{
                let f = pointer_lock_change.clone();
                move |e| {
                    let f = &mut *f.borrow_mut();
                    f (e);
                }
            }),
            EventListener::new(&document, "mozpointerlockchange",{
                let f = pointer_lock_change.clone();
                move |e| {
                    let f = &mut *f.borrow_mut();
                    f (e);
                }
            }),
            EventListener::new(&document, "webkitpointerlockchange",{
                let f = pointer_lock_change.clone();
                move |e| {
                    let f = &mut *f.borrow_mut();
                    f (e);
                }
            }),
            EventListener::new(&trigger, "click",request_lock)
        ));

        move || {
            let a = pointer_lock_change;
            event_listeners.take();
        }
    }

fn elements_are_equal(el1:&Element, el2:&Element) -> bool {
    //See: https://github.com/rustwasm/wasm-bindgen/issues/1672
    Object::is(&el1, &el2)
}
