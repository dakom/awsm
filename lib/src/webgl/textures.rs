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

// SimpleTexutreOptions represents the typical use case 
pub struct SimpleTextureOptions {
    pub flip_y: bool,
    pub wrap_s: TextureWrapMode,
    pub wrap_t: TextureWrapMode,
    pub filter_min: TextureMinFilter,
    pub filter_mag: TextureMagFilter,
    pub pixel_format: PixelFormat,
    pub data_type: DataType,
}

impl Default for SimpleTextureOptions {
    fn default() -> Self {
        Self {
            flip_y: true,
            wrap_s: TextureWrapMode::ClampToEdge,
            wrap_t: TextureWrapMode::ClampToEdge,
            filter_min: TextureMinFilter::Linear,
            filter_mag: TextureMagFilter::Linear,
            pixel_format: PixelFormat::Rgb,
            data_type: DataType::UnsignedByte,
        }
    }
}

//TODO - WebGL 2 allows a lot more options for internal and data formats than just PixelFormat
//(or maybe it's that PixelFormat has many more values)
pub struct TextureOptions {
    internal_format: PixelFormat, 
    data_format: PixelFormat,
    data_type: DataType,
}

//for some reason the function names are different for webgl 1 vs 2
cfg_if! {
    if #[cfg(feature = "webgl_1")] {
        fn get_texture_from_image_target(gl:&WebGlContext, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, image: &HtmlImageElement) -> Result<(), JsValue> {
            gl.tex_image_2d_with_u32_and_u32_and_image( bind_target, mip_level, internal_format, data_format, data_type, image)
        }
        fn get_texture_from_canvas_target(gl:&WebGlContext, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
            gl.tex_image_2d_with_u32_and_u32_and_canvas( bind_target, mip_level, internal_format, data_format, data_type, canvas)
        }
        fn get_texture_from_video_target(gl:&WebGlContext, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, video: &HtmlVideoElement) -> Result<(), JsValue> {
            gl.tex_image_2d_with_u32_and_u32_and_video( bind_target, mip_level, internal_format, data_format, data_type, video)
        }
    } else if #[cfg(feature = "webgl_2")] {
        fn get_texture_from_image_target(gl:&WebGlContext, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, image: &HtmlImageElement) -> Result<(), JsValue> {
            gl.tex_image_2d_with_u32_and_u32_and_html_image_element( bind_target, mip_level, internal_format, data_format, data_type, image)
        }
        fn get_texture_from_canvas_target(gl:&WebGlContext, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
            gl.tex_image_2d_with_u32_and_u32_and_html_canvas_element( bind_target, mip_level, internal_format, data_format, data_type, canvas)
        }
        fn get_texture_from_video_target(gl:&WebGlContext, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, video: &HtmlVideoElement) -> Result<(), JsValue> {
            gl.tex_image_2d_with_u32_and_u32_and_html_video_element( bind_target, mip_level, internal_format, data_format, data_type, video)
        }
    }
}

pub fn get_size (src:&WebGlTextureSource) -> (u32, u32) {
    match src {
        WebGlTextureSource::ArrayBufferView(_buffer, width, height) => {
            (*width as u32, *height as u32)
        },
        WebGlTextureSource::ByteArray(_buffer, width, height) => {
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
        internal_format: opts.pixel_format,
        data_format: opts.pixel_format,
        data_type: opts.data_type,
    }
}


//webgl2 allows mips for any texture, webgl1 is power of 2 only
#[cfg(feature = "webgl_1")] 
pub fn texture_sources_can_mipmap(srcs:&[&WebGlTextureSource]) -> Result<(), Error> {
    match srcs.iter().all(|&src| is_power_of_2(&src)) {
        true => Ok(()),
        false => Err(Error::from(NativeError::MipsPowerOf2))
    }
}
#[cfg(feature = "webgl_2")] 
pub fn texture_sources_can_mipmap(srcs:&[&WebGlTextureSource]) -> Result<(), Error> {
    Ok(()) 
}

pub fn assign_simple_texture_target(gl:&WebGlContext, bind_target: TextureTarget, opts:&SimpleTextureOptions, src:&WebGlTextureSource, dest:&WebGlTexture) -> Result<(), Error> {

    let set_parameters = Some(|_:&WebGlContext| {
        simple_parameters_target(&gl, bind_target, &opts, false);
    });

    assign_texture_target(&gl, bind_target, &get_texture_options_from_simple(&opts), set_parameters, &src, &dest)
}

pub fn assign_simple_texture_mips_target(gl:&WebGlContext, bind_target: TextureTarget, opts:&SimpleTextureOptions, srcs:&[&WebGlTextureSource], dest:&WebGlTexture) -> Result<(), Error> {

    texture_sources_can_mipmap(&srcs)?;
    let set_parameters = Some(|_:&WebGlContext| {
        simple_parameters_target(&gl, bind_target, &opts, true);
    });

    assign_texture_mips_target(&gl, bind_target, &get_texture_options_from_simple(&opts), set_parameters, &srcs, &dest)
}

pub fn simple_parameters_target(gl:&WebGlContext, bind_target: TextureTarget, opts:&SimpleTextureOptions, use_mips: bool) {

    let bind_target = bind_target as u32;

    if opts.flip_y {
        gl.pixel_storei(WebGlSpecific::UnpackFlipY as u32, 1);
    } else {
        gl.pixel_storei(WebGlSpecific::UnpackFlipY as u32, 0);
    }

    if use_mips {
        gl.generate_mipmap(bind_target);
    } else {
        gl.tex_parameteri(bind_target, TextureParameterName::TextureWrapS as u32, opts.wrap_s as i32); 
        gl.tex_parameteri(bind_target, TextureParameterName::TextureWrapT as u32, opts.wrap_t as i32); 
        gl.tex_parameteri(bind_target, TextureParameterName::TextureMinFilter as u32, opts.filter_min as i32); 
        gl.tex_parameteri(bind_target, TextureParameterName::TextureMagFilter as u32, opts.filter_mag as i32); 
    }
}

pub fn assign_texture_target(gl:&WebGlContext, bind_target: TextureTarget, opts:&TextureOptions,set_parameters:Option<impl Fn(&WebGlContext) -> ()>, src:&WebGlTextureSource, dest:&WebGlTexture) -> Result<(), Error> {
    assign_texture_mips_target(&gl, bind_target, &opts, set_parameters, &[src], &dest)
}

pub fn assign_texture_mips_target(gl:&WebGlContext, bind_target: TextureTarget, opts:&TextureOptions, set_parameters:Option<impl Fn(&WebGlContext) -> ()>, srcs:&[&WebGlTextureSource], dest:&WebGlTexture) -> Result<(), Error> {
    let bind_target = bind_target as u32;

    gl.bind_texture(bind_target, Some(dest));

    set_parameters.map(|f| f(&gl));

    for (mip_level, src) in srcs.iter().enumerate() {
        _assign_texture_target(&gl, bind_target, mip_level as i32, &opts, &src)?;
    }

    Ok(())
}

pub fn activate_texture_for_sampler_target(gl:&WebGlContext, bind_target: TextureTarget, sampler_index: usize, texture:&WebGlTexture) {
    gl.active_texture((TextureUnit::Texture0 as u32) + (sampler_index as u32));

    gl.bind_texture(bind_target as u32, Some(texture));
}

//internal use only
fn _assign_texture_target (gl:&WebGlContext, bind_target: u32, mip_level: i32, opts:&TextureOptions, src:&WebGlTextureSource) -> Result<(), Error> {

    let internal_format = opts.internal_format as i32;
    let data_format = opts.data_format as u32;
    let data_type = opts.data_type as u32;

    match src {
        WebGlTextureSource::ArrayBufferView(buffer_view, width, height) => {
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                bind_target,
                mip_level,
                internal_format,
                *width,
                *height,
                0,
                data_format,
                data_type,
                Some(buffer_view)
            )
        },
        WebGlTextureSource::ByteArray(buffer, width, height) => {
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                bind_target,
                mip_level,
                internal_format,
                *width,
                *height,
                0,
                data_format,
                data_type,
                Some(*buffer)
            )
        },
        WebGlTextureSource::ImageBitmap(bmp) => {
            gl.tex_image_2d_with_u32_and_u32_and_image_bitmap(
                bind_target,
                mip_level,
                internal_format,
                data_format,
                data_type,
                bmp
            )
        },
        WebGlTextureSource::ImageData(data) => {
            gl.tex_image_2d_with_u32_and_u32_and_image_data(
                bind_target,
                mip_level,
                internal_format,
                data_format,
                data_type,
                data
            )
        },
        WebGlTextureSource::ImageElement(img) => {
            get_texture_from_image_target(gl, bind_target, mip_level, internal_format, data_format, data_type, img)
        },
        WebGlTextureSource::CanvasElement(canvas) => {
            get_texture_from_canvas_target(gl, bind_target, mip_level, internal_format, data_format, data_type, canvas)
        },
        WebGlTextureSource::VideoElement(video) => {
            get_texture_from_video_target(gl, bind_target, mip_level, internal_format, data_format, data_type, video)
        },
        _ => Ok(())
    }.map_err(|err| Error::from(err))
}


fn is_power_of_2_val (val:u32) -> bool {
    val & (val -1) == 0
}
