// Thanks to Pauan!
// Reference: https://github.com/rustwasm/wasm-bindgen/issues/1126#issuecomment-451826093

use wasm_bindgen::prelude::*;
use web_sys::EventTarget;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::JsCast;

pub struct EventListener<'a, A> {
    node: EventTarget,
    kind: &'a str,
    callback: Closure<FnMut(A)>,
}

impl<'a, A> EventListener<'a, A> where A: FromWasmAbi + 'static {
    #[inline]
    pub fn new<F>(node: &EventTarget, kind: &'a str, f: F) -> Self where F: FnMut(A) + 'static {
        let callback = Closure::wrap(Box::new(f) as Box<FnMut(A)>);

        node.add_event_listener_with_callback(kind, callback.as_ref().unchecked_ref()).unwrap();

        Self {
            node: node.clone(),
            kind,
            callback,
        }
    }
}

impl<'a, A> Drop for EventListener<'a, A> {
    #[inline]
    fn drop(&mut self) {
        self.node.remove_event_listener_with_callback(self.kind, self.callback.as_ref().unchecked_ref()).unwrap();
    }
}