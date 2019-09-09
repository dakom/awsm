use crate::errors::{Error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext};

pub struct AudioPlayer {
    pub node: AudioBufferSourceNode,
    pub cb: Option<Closure<dyn FnMut() -> ()>>,
}

impl AudioPlayer {
    pub fn start<F>(
        ctx: &AudioContext,
        buffer: &AudioBuffer,
        on_ended: Option<F>,
    ) -> Result<Self, Error>
    where
        F: FnMut() -> () + 'static,
    {
        let node = ctx.create_buffer_source()?;

        node.set_buffer(Some(buffer));
        node.connect_with_audio_node(&ctx.destination())?;

        let cb: Option<Closure<dyn FnMut() -> ()>> = match on_ended {
            Some(f) => {
                let cb = Closure::wrap(Box::new(f) as Box<dyn FnMut() -> ()>);
                node.set_onended(Some(cb.as_ref().unchecked_ref()));
                Some(cb)
            }
            None => None,
        };

        node.start()?;

        Ok(Self { node, cb })
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.node.stop().unwrap();
        self.node.set_onended(None);
        self.cb.take();
    }
}
