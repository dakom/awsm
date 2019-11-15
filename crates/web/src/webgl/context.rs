use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

use super::attributes::PartialWebGlAttribute;
use super::buffers::PartialWebGlBuffer;
use super::drawing::PartialWebGlDrawing;
use super::enums::WebGlVersion;
use super::extensions::PartialWebGlExtensions;
use super::funcs::PartialWebGlFuncs;
use super::misc::PartialWebGlMisc;
use super::query::PartialWebGlQueries;
use super::shader::PartialWebGlShaders;
use super::textures::PartialWebGlTextures;
use super::toggles::PartialWebGlToggle;
use super::uniforms::PartialWebGlUniforms;
use super::viewport::PartialWebGlViewport;
use crate::errors::{Error, NativeError};
use serde::Serialize;

pub trait PartialWebGlVersion {
    fn awsm_get_version(&self) -> WebGlVersion;
}
impl PartialWebGlVersion for WebGlRenderingContext {
    fn awsm_get_version(&self) -> WebGlVersion {
        WebGlVersion::One
    }
}

impl PartialWebGlVersion for WebGl2RenderingContext {
    fn awsm_get_version(&self) -> WebGlVersion {
        WebGlVersion::Two
    }
}

pub trait PartialWebGlCanvas {
    fn awsm_get_canvas(&self) -> Result<HtmlCanvasElement, Error>;
}
impl PartialWebGlCanvas for WebGlRenderingContext {
    fn awsm_get_canvas(&self) -> Result<HtmlCanvasElement, Error> {
        self.canvas()
            .ok_or(Error::from(NativeError::WebGlCanvas))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_err| Error::from(NativeError::WebGlCanvas))
    }
}

impl PartialWebGlCanvas for WebGl2RenderingContext {
    fn awsm_get_canvas(&self) -> Result<HtmlCanvasElement, Error> {
        self.canvas()
            .ok_or(Error::from(NativeError::WebGlCanvas))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_err| Error::from(NativeError::WebGlCanvas))
    }
}
pub trait WebGlCommon:
    PartialWebGlVersion
    + PartialWebGlCanvas
    + PartialWebGlAttribute
    + PartialWebGlQueries
    + PartialWebGlBuffer
    + PartialWebGlDrawing
    + PartialWebGlExtensions
    + PartialWebGlFuncs
    + PartialWebGlMisc
    + PartialWebGlShaders
    + PartialWebGlTextures
    + PartialWebGlToggle
    + PartialWebGlUniforms
    + PartialWebGlViewport
{
}

impl WebGlCommon for WebGlRenderingContext {}

impl WebGlCommon for WebGl2RenderingContext {}

/* Creators */
pub struct WebGlContextOptions {
    pub alpha: bool,
    pub depth: bool,
    pub stencil: bool,
    pub antialias: bool,
    pub premultiplied_alpha: bool,
    pub preserve_drawing_buffer: bool,
    pub power_preference: PowerPreference,
    pub fail_if_major_performance_caveat: bool,
    pub desynchronized: bool,
}

pub enum PowerPreference {
    Default,
    HighPerformance,
    LowPower,
}

impl Default for WebGlContextOptions {
    fn default() -> Self {
        Self {
            alpha: true,
            depth: true,
            stencil: false,
            antialias: true,
            premultiplied_alpha: true,
            preserve_drawing_buffer: false,
            power_preference: PowerPreference::Default,
            fail_if_major_performance_caveat: false,
            desynchronized: false,
        }
    }
}

impl WebGlContextOptions {
    pub fn to_js_value(&self) -> JsValue {
        let power_preference = match self.power_preference {
            PowerPreference::LowPower => "low-power",
            PowerPreference::HighPerformance => "high-performance",
            _ => "default",
        };

        let sanitized = _WebGlContextOptions {
            alpha: self.alpha,
            depth: self.depth,
            stencil: self.stencil,
            antialias: self.antialias,
            premultiplied_alpha: self.premultiplied_alpha,
            preserve_drawing_buffer: self.preserve_drawing_buffer,
            power_preference,
            fail_if_major_performance_caveat: self.fail_if_major_performance_caveat,
            desynchronized: self.desynchronized,
        };

        serde_wasm_bindgen::to_value(&sanitized).unwrap()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct _WebGlContextOptions {
    alpha: bool,
    depth: bool,
    stencil: bool,
    antialias: bool,
    premultiplied_alpha: bool,
    preserve_drawing_buffer: bool,
    power_preference: &'static str,
    fail_if_major_performance_caveat: bool,
    desynchronized: bool,
}

pub fn get_webgl_context_1(
    canvas: &HtmlCanvasElement,
    opts: Option<&WebGlContextOptions>,
) -> Result<WebGlRenderingContext, Error> {

    #[cfg(feature = "debug_log")]
    log::info!("Webgl version 1");

    let context = match opts {
        Some(opts) => canvas.get_context_with_context_options("webgl", &opts.to_js_value()),
        None => canvas.get_context("webgl"),
    };

    context
        .and_then(|obj| match obj {
            None => Err(Error::Empty.into()),
            Some(ctx) => ctx
                .dyn_into::<web_sys::WebGlRenderingContext>()
                .map_err(|err| err.into()),
        })
        .map_err(|_| Error::Native(NativeError::WebGlContext))
}
pub fn get_webgl_context_2(
    canvas: &HtmlCanvasElement,
    opts: Option<&WebGlContextOptions>,
) -> Result<WebGl2RenderingContext, Error> {
    #[cfg(feature = "debug_log")]
    log::info!("Webgl version 2");

    let context = match opts {
        Some(opts) => canvas.get_context_with_context_options("webgl2", &opts.to_js_value()),
        None => canvas.get_context("webgl2"),
    };

    context
        .and_then(|obj| match obj {
            None => Err(Error::Empty.into()),
            Some(ctx) => ctx
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .map_err(|err| err.into()),
        })
        .map_err(|_| Error::Native(NativeError::WebGlContext))
}
