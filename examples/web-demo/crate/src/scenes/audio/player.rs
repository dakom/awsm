use crate::router::get_static_href;
use awsm_web::audio::{AudioPlayer};
use awsm_web::loaders::fetch;
use gloo_events::EventListener;
use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{AudioContext, Document, Element, HtmlElement, Window};

struct State {
    bg_loop: bool,
    oneshot: bool,
}

impl State {
    fn new() -> Self {
        Self {
            bg_loop: false,
            oneshot: false,
        }
    }
}

pub fn start(_window: Window, document: Document, body: HtmlElement) -> Result<(), JsValue> {
    let container: Element = document.create_element("div")?.into();
    container.set_class_name("audio-player");
    body.append_child(&container)?;

    let loading: Element = document.create_element("div")?.into();
    loading.set_class_name("audio-player-loading");
    loading.set_text_content(Some("loading audio..."));
    container.append_child(&loading)?;

    let ctx: AudioContext = AudioContext::new()?;
    let future = async move {
        let bg_loop_buffer = fetch::audio(&get_static_href("loop.mp3"), &ctx).await?;
        let one_shot_buffer = fetch::audio(&get_static_href("oneshot.mp3"), &ctx).await?;

        container.remove_child(&loading)?;

        let play_loop = create_button(&document, &container, "")?;
        let play_oneshot = create_button(&document, &container, "")?;

        let render_state = {
            let play_loop = play_loop.clone();
            let play_oneshot = play_oneshot.clone();
            move |state: &State| {
                match state.bg_loop {
                    true => play_loop.set_text_content(Some("stop loop")),
                    false => play_loop.set_text_content(Some("play loop")),
                };

                match state.oneshot {
                    true => play_oneshot.set_text_content(Some("stop oneshot")),
                    false => play_oneshot.set_text_content(Some("play oneshot")),
                };
            }
        };

        let state = Rc::new(RefCell::new(State::new()));
        render_state(&state.borrow());

        let mut bg_player: Option<AudioPlayer> = None;
        let mut oneshot_player: Option<Rc<RefCell<Option<AudioPlayer>>>> = None;

        let handle_loop = {
            let state = Rc::clone(&state);
            let render_state = render_state.clone();
            let ctx = ctx.clone();
            move |_: &_| {
                {
                    let mut state_obj = state.borrow_mut();
                    state_obj.bg_loop = !state_obj.bg_loop;
                    match state_obj.bg_loop {
                        true => {
                            info!("should be playing loop...");

                            let player = AudioPlayer::play(
                                &ctx,
                                &bg_loop_buffer,
                                Some({
                                    let state = Rc::clone(&state);
                                    let render_state = render_state.clone();
                                    move || {
                                        info!("loop ended!");
                                        //this won't ever actually happen
                                        let mut state = state.borrow_mut();
                                        state.bg_loop = false;
                                        render_state(&state);
                                    }
                                }),
                            )
                            .unwrap();

                            player.node.set_loop(true);

                            bg_player = Some(player);
                        }
                        false => {
                            bg_player.take();
                        }
                    }
                }
                render_state(&state.borrow());
            }
        };

        let handle_oneshot = {
            let state = Rc::clone(&state);
            let render_state = render_state.clone();
            let ctx = ctx.clone();
            move |_: &_| {
                {
                    let mut state_obj = state.borrow_mut();
                    state_obj.oneshot = !state_obj.oneshot;
                    match state_obj.oneshot {
                        true => {
                            info!("should be playing oneshot...");
                            let player = AudioPlayer::play_oneshot(
                                &ctx,
                                &one_shot_buffer,
                                Some({
                                    let state = Rc::clone(&state);
                                    let render_state = render_state.clone();
                                    move || {
                                        info!("oneshot ended!");
                                        let mut state = state.borrow_mut();
                                        state.oneshot = false;
                                        render_state(&state);
                                    }
                                }),
                            )
                            .unwrap();

                            oneshot_player = Some(player);
                        }
                        false => {
                            if let Some(player) = oneshot_player.take() {
                                player.borrow_mut().take();
                            }
                        }
                    }
                }
                render_state(&state.borrow());
            }
        };

        EventListener::new(&play_loop, "click", handle_loop).forget();
        EventListener::new(&play_oneshot, "click", handle_oneshot).forget();

        Ok(JsValue::null())
    };

    //we don't handle errors here because they are exceptions
    //hope you're running in an environment where uncaught rejects/exceptions are reported!
    future_to_promise(future);

    Ok(())
}

fn create_button(document: &Document, root: &Element, label: &str) -> Result<HtmlElement, JsValue> {
    let item: HtmlElement = document.create_element("div")?.dyn_into()?;
    item.set_class_name("button audio-player-button");
    item.set_text_content(Some(label));
    root.append_child(&item)?;
    Ok(item)
}
