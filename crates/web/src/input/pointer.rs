use js_sys::Object;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, Event, EventTarget, MouseEvent};

pub struct PointerLock<'a> {
    pub trigger: &'a EventTarget,
    pub target: &'a EventTarget,
    click_cb: Closure<dyn FnMut(&Event)>,
    change_cb: Closure<dyn FnMut(&Event)>,
    change_cb_moz: Closure<dyn FnMut(&Event)>,
    change_cb_webkit: Closure<dyn FnMut(&Event)>,
    document: &'a Document,
}

impl<'a> PointerLock<'a> {
    pub fn start<F, G, H>(
        trigger: &'a Element,
        target: &'a Element,
        document: &'a Document,
        on_start: F,
        on_move: G,
        on_end: H,
    ) -> Self
    where
        F: Fn(&Element) + 'static,
        G: Fn(i32, i32) + 'static,
        H: Fn() + 'static,
    {
        let is_locked = Rc::new(Cell::new(false));

        let request_lock = {
            let target = target.clone();
            let is_locked = Rc::clone(&is_locked);
            move |_: &_| {
                if !is_locked.get() {
                    target.request_pointer_lock();
                    is_locked.set(true);
                }
            }
        };

        let on_pointer_move = Rc::new(move |evt: &Event| {
            let evt: MouseEvent = JsValue::from(evt).into();
            let (x, y) = (evt.movement_x(), evt.movement_y());

            on_move(x, y);
        });

        let pointer_lock_change = {
            let target = target.clone();
            let document = document.clone();
            let is_locked = Rc::clone(&is_locked);

            let mut listener: Option<Closure<dyn FnMut(&Event)>> = None;

            Rc::new(RefCell::new(move |_initial_evt: &Event| {
                let lock_enabled = match document.pointer_lock_element() {
                    None => false,
                    Some(element) => elements_are_equal(&element, &target),
                };
                is_locked.set(lock_enabled);

                if lock_enabled {
                    on_start(&target);

                    
                    #[cfg(feature = "debug_log")]
                    log::info!("pointer lock enabled!");

                    let move_cb = Closure::wrap(Box::new({
                        let f = Rc::clone(&on_pointer_move);
                        move |e: &Event| f(e)
                    }) as Box<dyn FnMut(&Event)>);
                    document
                        .add_event_listener_with_callback(
                            "mousemove",
                            move_cb.as_ref().unchecked_ref(),
                        )
                        .unwrap_throw();
                    listener = Some(move_cb);
                } else {
                    on_end();
                    if let Some(l) = listener.take() {
                        document
                            .remove_event_listener_with_callback(
                                "mousemove",
                                l.as_ref().unchecked_ref(),
                            )
                            .unwrap_throw();
                    }
                }
            }))
        };

        let click_cb = Closure::wrap(Box::new(request_lock) as Box<dyn FnMut(&Event)>);
        trigger
            .add_event_listener_with_callback("click", click_cb.as_ref().unchecked_ref())
            .unwrap_throw();

        let change_cb = Closure::wrap(Box::new({
            let f = pointer_lock_change.clone();
            move |e: &Event| {
                let f = &mut *f.borrow_mut();
                f(e);
            }
        }) as Box<dyn FnMut(&Event)>);
        document
            .add_event_listener_with_callback(
                "pointerlockchange",
                change_cb.as_ref().unchecked_ref(),
            )
            .unwrap_throw();

        let change_cb_moz = Closure::wrap(Box::new({
            let f = pointer_lock_change.clone();
            move |e: &Event| {
                let f = &mut *f.borrow_mut();
                f(e);
            }
        }) as Box<dyn FnMut(&Event)>);
        document
            .add_event_listener_with_callback(
                "mozpointerlockchange",
                change_cb_moz.as_ref().unchecked_ref(),
            )
            .unwrap_throw();
        let change_cb_webkit = Closure::wrap(Box::new({
            let f = pointer_lock_change.clone();
            move |e: &Event| {
                let f = &mut *f.borrow_mut();
                f(e);
            }
        }) as Box<dyn FnMut(&Event)>);
        document
            .add_event_listener_with_callback(
                "webkitpointerlockchange",
                change_cb_webkit.as_ref().unchecked_ref(),
            )
            .unwrap_throw();

        Self {
            click_cb,
            change_cb,
            change_cb_moz,
            change_cb_webkit,
            target,
            trigger,
            document,
        }
    }
}

impl<'a> Drop for PointerLock<'a> {
    fn drop(&mut self) {
        self.trigger
            .remove_event_listener_with_callback("click", self.click_cb.as_ref().unchecked_ref())
            .unwrap_throw();
        self.document
            .remove_event_listener_with_callback(
                "pointerlockchange",
                self.change_cb.as_ref().unchecked_ref(),
            )
            .unwrap_throw();
        self.document
            .remove_event_listener_with_callback(
                "mozpointerlockchange",
                self.change_cb_moz.as_ref().unchecked_ref(),
            )
            .unwrap_throw();
        self.document
            .remove_event_listener_with_callback(
                "webkitpointerlockchange",
                self.change_cb_webkit.as_ref().unchecked_ref(),
            )
            .unwrap_throw();

        #[cfg(feature = "debug_log")]
        log::info!("dropped pointer lock!");
    }
}

fn elements_are_equal(el1: &Element, el2: &Element) -> bool {
    //See: https://github.com/rustwasm/wasm-bindgen/issues/1672
    Object::is(&el1, &el2)
}
