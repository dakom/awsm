use crate::errors::{Error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext};
use std::rc::Rc;
use std::cell::RefCell;

pub struct AudioPlayer {
    pub node: AudioBufferSourceNode,
    pub cb: Option<Closure<dyn FnMut() -> ()>>,
}

impl AudioPlayer {
    pub fn play<F>(
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

    //A regular audio player is effectively a one-shot since dropping will stop it
    //But it can be annoying to need to keep it around in memory until playing is finished
    //So this one-shot will drop itself when finished
    //It can still be force-dropped by calling borrow_mut().take on the result (see example)
    pub fn play_oneshot<F>(
        ctx: &AudioContext,
        buffer: &AudioBuffer,
        on_ended: Option<F>,
    ) -> Result<Rc<RefCell<Option<AudioPlayer>>>, Error>
    where
        F: FnMut() -> () + 'static,
    {
        let player = Rc::new(RefCell::new(None));
        let on_ended = Rc::new(RefCell::new(on_ended));

        let _player = AudioPlayer::play(ctx, buffer, Some({
            let player = Rc::clone(&player);
            move || {
                on_ended.borrow_mut().as_mut().map(|cb| cb());
                player.borrow_mut().take();
            }
        }))?;

        *player.borrow_mut() = Some(_player);

        Ok(player)
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.node.stop().unwrap();
        self.node.set_onended(None);
        self.cb.take();
    }
}


pub struct AudioOneShot {
    pub player: Rc<RefCell<Option<AudioPlayer>>>,
}

impl AudioOneShot {
    pub fn play<F>(
        ctx: &AudioContext,
        buffer: &AudioBuffer,
        on_ended: Option<F>,
    ) -> Result<Self, Error>
    where
        F: FnMut() -> () + 'static,
    {

        let player = Rc::new(RefCell::new(None));
        let on_ended = Rc::new(RefCell::new(on_ended));

        let _player = AudioPlayer::play(ctx, buffer, Some({
            let player = Rc::clone(&player);
            move || {
                on_ended.borrow_mut().as_mut().map(|cb| cb());
                player.borrow_mut().take();
            }
        }))?;

        *player.borrow_mut() = Some(_player);

        Ok(Self{
            player
        })
    }

}