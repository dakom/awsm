use crate::errors::{Error};
use crate::renderer::{setup_renderer};
use crate::input::{setup_input};
use crate::audio::{setup_audio};
use crate::time::{setup_time};
use crate::viewport::{setup_viewport};
use crate::scene::{setup_scene};
use crate::gltf::{load_gltf, setup_gltf, GltfResource, GltfFileType, GltfSceneRoot};
use specs::{World, WorldExt};
use futures::{Future};
use web_sys::{
    Window,
    HtmlCanvasElement,
    Document,
    Element,
    AudioContext,
};
use awsm::webgl::{
    ClearBufferMask,
    WebGl2Renderer,
    WebGlContextOptions,
    get_webgl_context_2,
    Id,
};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Engine {
    world: World
}

impl Engine {
    pub fn new(
        window: &Window,
        document: &Document,
        canvas: &HtmlCanvasElement, 
        audio_context: &AudioContext,
        webgl_opts: Option<&WebGlContextOptions>, 
        clear_color:Option<(f32, f32, f32, f32)>,
        pointer_lock:Option<(&Element, &Element)>, //trigger, target
        gltf: Option<(GltfResource, GltfSceneRoot)>,
    ) -> Result<Self, Error> {

        let mut world = WorldExt::new();

        //each of these should return a system maybe
        //input, time, and screen size should be resources
        setup_input(&mut world, window, document, canvas, pointer_lock)?;
        setup_time(&mut world)?;
        setup_audio(&mut world, audio_context)?;
        setup_viewport(&mut world, window)?;
        setup_renderer(&mut world, canvas, webgl_opts, clear_color)?;
        setup_scene(&mut world)?;

        if let Some((gltf_resource, scene_root)) = gltf {
            setup_gltf(&mut world, gltf_resource, scene_root)?;
        }

        Ok(Self{
            world
        })
    }

    pub fn world(&mut self) -> &mut World {
        &mut self.world
    }

}
