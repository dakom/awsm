use crate::errors::{Error};
use specs::{World, WorldExt};
use web_sys::{
    Window,
    HtmlCanvasElement,
    Document,
    Element,
    AudioContext,
};

pub fn setup_input(
    world:&mut World,
    window: &Window,
    document: &Document,
    canvas: &HtmlCanvasElement, 
    pointer_lock:Option<(&Element, &Element)>, //trigger, target
) -> Result<(), Error> {


    Ok(())
}
