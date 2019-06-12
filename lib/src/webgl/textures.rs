use web_sys::{WebGlTexture, ImageBitmap, ImageData, HtmlImageElement, HtmlCanvasElement, HtmlVideoElement};
use super::context::{WebGlContext};
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use js_sys::{Object};
use crate::errors::{Error, NativeError};
use super::enums::{TextureUnit, TextureParameterName, TextureWrapMode, TextureMinFilter, TextureMagFilter, TextureTarget, PixelFormat, DataType, WebGlSpecific};
use cfg_if::cfg_if;

pub enum WebGlTextureSource <'a> {
    ArrayBufferView(&'a Object, i32, i32),
    ByteArray(&'a [u8], i32, i32),
    ImageBitmap(&'a ImageBitmap),
    ImageData(&'a ImageData),
    ImageElement(&'a HtmlImageElement),
    CanvasElement(&'a HtmlCanvasElement),
    VideoElement(&'a HtmlVideoElement),
}

// SimpleTexutreOptions uses enums that represent the typical use case
// in order to support any possible options without making the wrappers
// too verbose, TextureOptions itself uses plain scalars
pub struct SimpleTextureOptions {
    pub flipY: bool,
    pub wrapS: TextureWrapMode,
    pub wrapT: TextureWrapMode,
    pub filterMin: TextureMinFilter,
    pub filterMag: TextureMagFilter,
    pub pixelFormat: PixelFormat,
    pub dataType: DataType,
}

impl Default for SimpleTextureOptions {
    fn default() -> Self {
        Self {
            flipY: true,
            wrapS: TextureWrapMode::ClampToEdge,
            wrapT: TextureWrapMode::ClampToEdge,
            filterMin: TextureMinFilter::Linear,
            filterMag: TextureMagFilter::Linear,
            pixelFormat: PixelFormat::Rgb,
            dataType: DataType::UnsignedByte,
        }
    }
}

pub struct TextureOptions {
    internalFormat: i32, 
    dataFormat: u32,
    dataType: u32,
}


pub fn get_size (src:&WebGlTextureSource) -> (u32, u32) {
    match src {
        WebGlTextureSource::ArrayBufferView(buffer, width, height) => {
            (*width as u32, *height as u32)
        },
        WebGlTextureSource::ByteArray(buffer, width, height) => {
            (*width as u32, *height as u32)
        },
        WebGlTextureSource::ImageBitmap(bmp) => {
            (bmp.width(), bmp.height())
        },
        WebGlTextureSource::ImageData(data) => {
            (data.width(), data.height())
        },
        WebGlTextureSource::ImageElement(img) => {
            (img.width(), img.height())
        },
        WebGlTextureSource::CanvasElement(canvas) => {
            (canvas.width(), canvas.height())
        },
        WebGlTextureSource::VideoElement(video) => {
            (video.width(), video.height())
        },
    }
}

pub fn is_power_of_2 (src:&WebGlTextureSource) -> bool {
    let (width, height) = get_size(&src);
    is_power_of_2_val(width) && is_power_of_2_val(height)
}

fn get_texture_options_from_simple(opts:&SimpleTextureOptions) -> TextureOptions {
    TextureOptions {
        internalFormat: opts.pixelFormat as i32,
        dataFormat: opts.pixelFormat as u32,
        dataType: opts.dataType as u32,
    }
}


pub fn assign_simple_texture (gl:&WebGlContext, bind_target: TextureTarget, opts:&SimpleTextureOptions, src:&WebGlTextureSource, dest:&WebGlTexture) -> Result<(), Error> {

    let set_parameters = Some(|_:&WebGlContext| {
        simple_parameters (&gl, bind_target, &opts, false);
    });

    assign_texture(&gl, bind_target, &get_texture_options_from_simple(&opts), set_parameters, &src, &dest)
}

pub fn assign_simple_texture_mips (gl:&WebGlContext, bind_target: TextureTarget, opts:&SimpleTextureOptions, srcs:&[&WebGlTextureSource], dest:&WebGlTexture) -> Result<(), Error> {

    let set_parameters = Some(|_:&WebGlContext| {
        simple_parameters (&gl, bind_target, &opts, true);
    });

    assign_texture_mips(&gl, bind_target, &get_texture_options_from_simple(&opts), set_parameters, &srcs, &dest)
}

pub fn simple_parameters (gl:&WebGlContext, bind_target: TextureTarget, opts:&SimpleTextureOptions, use_mips: bool) {

    let bind_target = bind_target as u32;

    if opts.flipY {
        gl.pixel_storei(WebGlSpecific::UnpackFlipY as u32, 1);
    } else {
        gl.pixel_storei(WebGlSpecific::UnpackFlipY as u32, 0);
    }

    if use_mips {
        gl.generate_mipmap(bind_target);
    } else {
        gl.tex_parameteri(bind_target, TextureParameterName::TextureWrapS as u32, opts.wrapS as i32); 
        gl.tex_parameteri(bind_target, TextureParameterName::TextureWrapT as u32, opts.wrapT as i32); 
        gl.tex_parameteri(bind_target, TextureParameterName::TextureMinFilter as u32, opts.filterMin as i32); 
        gl.tex_parameteri(bind_target, TextureParameterName::TextureMagFilter as u32, opts.filterMag as i32); 
    }
}

pub fn assign_texture (gl:&WebGlContext, bind_target: TextureTarget, opts:&TextureOptions,set_parameters:Option<impl Fn(&WebGlContext) -> ()>, src:&WebGlTextureSource, dest:&WebGlTexture) -> Result<(), Error> {
    assign_texture_mips(&gl, bind_target, &opts, set_parameters, &[src], &dest)
}

pub fn assign_texture_mips (gl:&WebGlContext, bind_target: TextureTarget, opts:&TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, srcs:&[&WebGlTextureSource], dest:&WebGlTexture) -> Result<(), Error> {

    //untested but should be right
    //webgl2 allows mips for any texture, webgl1 is power of 2 only
    cfg_if! {
        if #[cfg(feature = "webgl1")] {
            fn sanity_check() -> Result<(), Error> {
                if !srcs.iter().all(|&src| is_power_of_2(&src)) {
                    Err(Error::from(NativeError::MipsPowerOf2))
                } else {
                    Ok(())
                }
            }
        } else {
            fn sanity_check() -> Result<(), Error> {
                Ok(())
            }
        }
    }

    sanity_check()?;

    let bind_target = bind_target as u32;

    gl.bind_texture(bind_target, Some(dest));

    set_parameters.map(|f| f(&gl));

    for (mip_level, src) in srcs.iter().enumerate() {
        _assign_texture(&gl, bind_target, mip_level as i32, &opts, &src, &dest)?;
    }

    Ok(())
}

pub fn activate_texture_for_sampler(gl:&WebGlContext, bind_target: TextureTarget, sampler_index: u32, texture:&WebGlTexture) {
    gl.active_texture(TextureUnit::Texture0 as u32 + sampler_index);

    gl.bind_texture(bind_target as u32, Some(texture));
}

//internal use only
fn _assign_texture (gl:&WebGlContext, bind_target: u32, mip_level: i32, opts:&TextureOptions, src:&WebGlTextureSource, dest:&WebGlTexture) -> Result<(), Error> {


    //TODO - call the stuff in
    // https://github.com/dakom/awsm-typescript/blob/master/src/lib/exports/webgl/WebGl-Textures.ts#L96
    match src {
        WebGlTextureSource::ArrayBufferView(buffer_view, width, height) => {
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                bind_target,
                mip_level,
                opts.internalFormat,
                *width,
                *height,
                0,
                opts.dataFormat,
                opts.dataType,
                Some(buffer_view)
            )
        },
        WebGlTextureSource::ByteArray(buffer, width, height) => {
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                bind_target,
                mip_level,
                opts.internalFormat,
                *width,
                *height,
                0,
                opts.dataFormat,
                opts.dataType,
                Some(*buffer)
            )
        },
        WebGlTextureSource::ImageBitmap(bmp) => {
            gl.tex_image_2d_with_u32_and_u32_and_image_bitmap(
                bind_target,
                mip_level,
                opts.internalFormat,
                opts.dataFormat,
                opts.dataType,
                bmp
            )
        },
        WebGlTextureSource::ImageData(data) => {
            gl.tex_image_2d_with_u32_and_u32_and_image_data(
                bind_target,
                mip_level,
                opts.internalFormat,
                opts.dataFormat,
                opts.dataType,
                data
            )
        },
        WebGlTextureSource::ImageElement(img) => {
             gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                bind_target,
                mip_level,
                opts.internalFormat,
                opts.dataFormat,
                opts.dataType,
                img
            )
        },
        WebGlTextureSource::CanvasElement(canvas) => {
            gl.tex_image_2d_with_u32_and_u32_and_html_canvas_element(
                bind_target,
                mip_level,
                opts.internalFormat,
                opts.dataFormat,
                opts.dataType,
                canvas
            )
        },
        WebGlTextureSource::VideoElement(video) => {
            gl.tex_image_2d_with_u32_and_u32_and_html_video_element(
                bind_target,
                mip_level,
                opts.internalFormat,
                opts.dataFormat,
                opts.dataType,
                video
            )
        },
        _ => Ok(())
    }.map_err(|err| Error::from(err))
}


fn is_power_of_2_val (val:u32) -> bool {
    val & (val -1) == 0
}
