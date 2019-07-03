use web_sys::{WebGlTexture, ImageBitmap, ImageData, HtmlImageElement, HtmlCanvasElement, HtmlVideoElement};
use web_sys::{WebGlRenderingContext,WebGl2RenderingContext};
use wasm_bindgen::prelude::JsValue;
use js_sys::{Object};
use crate::errors::{Error, NativeError};
use super::{Id, WebGlCommon, WebGlRenderer, TextureUnit, TextureParameterName, TextureWrapMode, TextureMinFilter, TextureMagFilter, TextureTarget, PixelFormat, DataType, WebGlSpecific};


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

pub trait PartialWebGlTextures {
    fn awsm_create_texture(&self) -> Result<WebGlTexture, Error>;

    fn awsm_get_texture_from_image_target(&self, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, image: &HtmlImageElement) -> Result<(), JsValue>;
    fn awsm_get_texture_from_canvas_target(&self, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, canvas: &HtmlCanvasElement) -> Result<(), JsValue>;
    fn awsm_get_texture_from_video_target(&self, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, video: &HtmlVideoElement) -> Result<(), JsValue>;

    fn awsm_texture_sources_can_mipmap(&self, srcs:&[&WebGlTextureSource]) -> Result<(), Error>;

    fn awsm_assign_simple_texture_target(&self, bind_target: TextureTarget, opts:&SimpleTextureOptions, src:&WebGlTextureSource, dest:&WebGlTexture) -> Result<(), Error>;

    fn awsm_assign_simple_texture_target_mips(&self, bind_target: TextureTarget, opts:&SimpleTextureOptions, srcs:&[&WebGlTextureSource], dest:&WebGlTexture) -> Result<(), Error>;

    fn awsm_simple_parameters_target(&self, bind_target: TextureTarget, opts:&SimpleTextureOptions, use_mips: bool);

    fn awsm_assign_texture_target(&self, bind_target: TextureTarget, opts:&TextureOptions,set_parameters:Option<impl Fn(&Self) -> ()>, src:&WebGlTextureSource, dest:&WebGlTexture) -> Result<(), Error>;

    fn awsm_assign_texture_target_mips(&self, bind_target: TextureTarget, opts:&TextureOptions, set_parameters:Option<impl Fn(&Self) -> ()>, srcs:&[&WebGlTextureSource], dest:&WebGlTexture) -> Result<(), Error>;

    fn awsm_activate_texture_for_sampler_target_index(&self, bind_target: TextureTarget, sampler_index: u32, texture:&WebGlTexture);

    fn _awsm_assign_texture_target(&self, bind_target: u32, mip_level: i32, opts:&TextureOptions, src:&WebGlTextureSource) -> Result<(), Error>;
}

macro_rules! impl_context {
    ($($type:ty { $($defs:tt)* })+) => {
        $(impl PartialWebGlTextures for $type {

            fn awsm_create_texture(&self) -> Result<WebGlTexture, Error> {
                self.create_texture().ok_or(Error::from(NativeError::NoCreateTexture))
            }

            fn awsm_assign_simple_texture_target(&self, bind_target: TextureTarget, opts:&SimpleTextureOptions, src:&WebGlTextureSource, dest:&WebGlTexture) -> Result<(), Error> {

                let set_parameters = Some(|_:&$type| {
                    self.awsm_simple_parameters_target(bind_target, &opts, false);
                });

                self.awsm_assign_texture_target(bind_target, &get_texture_options_from_simple(&opts), set_parameters, &src, &dest)
            }

            fn awsm_assign_simple_texture_target_mips(&self, bind_target: TextureTarget, opts:&SimpleTextureOptions, srcs:&[&WebGlTextureSource], dest:&WebGlTexture) -> Result<(), Error> {

                self.awsm_texture_sources_can_mipmap(&srcs)?;
                let set_parameters = Some(|_:&$type| {
                    self.awsm_simple_parameters_target(bind_target, &opts, true);
                });

                self.awsm_assign_texture_target_mips(bind_target, &get_texture_options_from_simple(&opts), set_parameters, &srcs, &dest)
            }

            fn awsm_simple_parameters_target(&self, bind_target: TextureTarget, opts:&SimpleTextureOptions, use_mips: bool) {

                let bind_target = bind_target as u32;

                if opts.flip_y {
                    self.pixel_storei(WebGlSpecific::UnpackFlipY as u32, 1);
                } else {
                    self.pixel_storei(WebGlSpecific::UnpackFlipY as u32, 0);
                }

                if use_mips {
                    self.generate_mipmap(bind_target);
                } else {
                    self.tex_parameteri(bind_target, TextureParameterName::TextureWrapS as u32, opts.wrap_s as i32); 
                    self.tex_parameteri(bind_target, TextureParameterName::TextureWrapT as u32, opts.wrap_t as i32); 
                    self.tex_parameteri(bind_target, TextureParameterName::TextureMinFilter as u32, opts.filter_min as i32); 
                    self.tex_parameteri(bind_target, TextureParameterName::TextureMagFilter as u32, opts.filter_mag as i32); 
                }
            }

            fn awsm_assign_texture_target(&self, bind_target: TextureTarget, opts:&TextureOptions,set_parameters:Option<impl Fn(&Self) -> ()>, src:&WebGlTextureSource, dest:&WebGlTexture) -> Result<(), Error> {
                self.awsm_assign_texture_target_mips(bind_target, &opts, set_parameters, &[src], &dest)
            }


            fn awsm_assign_texture_target_mips(&self, bind_target: TextureTarget, opts:&TextureOptions, set_parameters:Option<impl Fn(&Self) -> ()>, srcs:&[&WebGlTextureSource], dest:&WebGlTexture) -> Result<(), Error> {
                let bind_target = bind_target as u32;

                self.bind_texture(bind_target, Some(dest));

                set_parameters.map(|f| f(self));

                for (mip_level, src) in srcs.iter().enumerate() {
                    self._awsm_assign_texture_target(bind_target, mip_level as i32, &opts, &src)?;
                }

                Ok(())
            }

            fn awsm_activate_texture_for_sampler_target_index(&self, bind_target: TextureTarget, sampler_index: u32, texture:&WebGlTexture) {
                self.active_texture((TextureUnit::Texture0 as u32) + sampler_index );

                self.bind_texture(bind_target as u32, Some(texture));
            }

            //internal use only
            fn _awsm_assign_texture_target(&self, bind_target: u32, mip_level: i32, opts:&TextureOptions, src:&WebGlTextureSource) -> Result<(), Error> {

                let internal_format = opts.internal_format as i32;
                let data_format = opts.data_format as u32;
                let data_type = opts.data_type as u32;

                match src {
                    WebGlTextureSource::ArrayBufferView(buffer_view, width, height) => {
                        self.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
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
                        self.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
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
                        self.tex_image_2d_with_u32_and_u32_and_image_bitmap(
                            bind_target,
                            mip_level,
                            internal_format,
                            data_format,
                            data_type,
                            bmp
                            )
                    },
                    WebGlTextureSource::ImageData(data) => {
                        self.tex_image_2d_with_u32_and_u32_and_image_data(
                            bind_target,
                            mip_level,
                            internal_format,
                            data_format,
                            data_type,
                            data
                            )
                    },
                    WebGlTextureSource::ImageElement(img) => {
                        self.awsm_get_texture_from_image_target(bind_target, mip_level, internal_format, data_format, data_type, img)
                    },
                    WebGlTextureSource::CanvasElement(canvas) => {
                        self.awsm_get_texture_from_canvas_target(bind_target, mip_level, internal_format, data_format, data_type, canvas)
                    },
                    WebGlTextureSource::VideoElement(video) => {
                        self.awsm_get_texture_from_video_target(bind_target, mip_level, internal_format, data_format, data_type, video)
                    },
                }.map_err(|err| Error::from(err))
            }

            $($defs)*
        })+
    };
}

impl_context!{
    WebGlRenderingContext{
        fn awsm_get_texture_from_image_target(&self, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, image: &HtmlImageElement) -> Result<(), JsValue> {
            self.tex_image_2d_with_u32_and_u32_and_image( bind_target, mip_level, internal_format, data_format, data_type, image)
        }
        fn awsm_get_texture_from_canvas_target(&self, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
            self.tex_image_2d_with_u32_and_u32_and_canvas( bind_target, mip_level, internal_format, data_format, data_type, canvas)
        }
        fn awsm_get_texture_from_video_target(&self, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, video: &HtmlVideoElement) -> Result<(), JsValue> {
            self.tex_image_2d_with_u32_and_u32_and_video( bind_target, mip_level, internal_format, data_format, data_type, video)
        }

        fn awsm_texture_sources_can_mipmap(&self, srcs:&[&WebGlTextureSource]) -> Result<(), Error> {
            match srcs.iter().all(|&src| is_power_of_2(&src)) {
                true => Ok(()),
                false => Err(Error::from(NativeError::MipsPowerOf2))
            }
        }
    }

    WebGl2RenderingContext{
        fn awsm_get_texture_from_image_target(&self, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, image: &HtmlImageElement) -> Result<(), JsValue> {
            self.tex_image_2d_with_u32_and_u32_and_html_image_element( bind_target, mip_level, internal_format, data_format, data_type, image)
        }
        fn awsm_get_texture_from_canvas_target(&self, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
            self.tex_image_2d_with_u32_and_u32_and_html_canvas_element( bind_target, mip_level, internal_format, data_format, data_type, canvas)
        }
        fn awsm_get_texture_from_video_target(&self, bind_target: u32, mip_level: i32, internal_format: i32, data_format: u32, data_type: u32, video: &HtmlVideoElement) -> Result<(), JsValue> {
            self.tex_image_2d_with_u32_and_u32_and_html_video_element( bind_target, mip_level, internal_format, data_format, data_type, video)
        }

        fn awsm_texture_sources_can_mipmap(&self, _:&[&WebGlTextureSource]) -> Result<(), Error> {
            Ok(()) 
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

fn is_power_of_2_val (val:u32) -> bool {
    val & (val -1) == 0
}


//WebGlRenderer Impl
pub(super) struct TextureSamplerInfo {
    bind_target: TextureTarget,
    texture_id: Id,
}

impl <T: WebGlCommon> WebGlRenderer<T> {

    pub fn create_texture(&mut self) -> Result<Id, Error> {
        let texture = self.gl.awsm_create_texture()?;

        let id = self.texture_lookup.insert(texture);

        Ok(id)
    }

    //public interfaces here are simple wrappers to pass the texture target along

    pub fn assign_simple_texture(&mut self, texture_id:Id, opts:&SimpleTextureOptions, src:&WebGlTextureSource) -> Result<(), Error> {
        self.assign_simple_texture_target(texture_id, TextureTarget::Texture2D, &opts, &src)
    }
    pub fn assign_simple_texture_mips(&mut self, texture_id:Id, opts:&SimpleTextureOptions, srcs:&[&WebGlTextureSource]) -> Result<(), Error> {
        self.assign_simple_texture_target_mips(texture_id, TextureTarget::Texture2D, &opts, &srcs)
    }
    pub fn assign_texture(&mut self, texture_id: Id, opts:&TextureOptions, set_parameters:Option<impl Fn(&T) -> ()>, src:&WebGlTextureSource) -> Result<(), Error> {
        self.assign_texture_target(texture_id, TextureTarget::Texture2D, &opts, set_parameters, &src)
    }
    pub fn assign_texture_mips(&mut self, texture_id: Id, opts:&TextureOptions, set_parameters:Option<impl Fn(&T) -> ()>, srcs:&[&WebGlTextureSource]) -> Result<(), Error> {
        self.assign_texture_target_mips(texture_id, TextureTarget::Texture2D, &opts, set_parameters, &srcs)
    }

    pub fn activate_texture_for_sampler(&mut self, texture_id: Id, sampler_name: &str) -> Result<(), Error> {
        self.activate_texture_for_sampler_target(TextureTarget::Texture2D, texture_id, sampler_name)
    }

    pub fn activate_texture_for_sampler_index(&mut self, texture_id: Id, sampler_index: u32) -> Result<(), Error> {
        self.activate_texture_for_sampler_target_index(TextureTarget::Texture2D, texture_id, sampler_index)
    }

    //Texture assigning will bind the texture - so the slot for activations effectively becomes None 
    pub fn assign_simple_texture_target(&mut self, texture_id:Id, bind_target: TextureTarget, opts:&SimpleTextureOptions, src:&WebGlTextureSource) -> Result<(), Error> {
        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id = Some(texture_id);
        self.current_texture_slot = None;

        self.gl.awsm_assign_simple_texture_target(bind_target, &opts, &src, &texture)

    }

    pub fn assign_simple_texture_target_mips(&mut self, texture_id:Id, bind_target: TextureTarget, opts:&SimpleTextureOptions, srcs:&[&WebGlTextureSource]) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id = Some(texture_id);
        self.current_texture_slot = None;

        self.gl.awsm_assign_simple_texture_target_mips(bind_target, &opts, &srcs, &texture)
    }


    pub fn assign_texture_target(&mut self, texture_id: Id, bind_target: TextureTarget, opts:&TextureOptions, set_parameters:Option<impl Fn(&T) -> ()>, src:&WebGlTextureSource) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;

        self.current_texture_id = Some(texture_id);
        self.current_texture_slot = None;

        self.gl.awsm_assign_texture_target(bind_target, &opts, set_parameters, &src, &texture)
    }

    pub fn assign_texture_target_mips(&mut self, texture_id: Id, bind_target: TextureTarget, opts:&TextureOptions, set_parameters:Option<impl Fn(&T) -> ()>, srcs:&[&WebGlTextureSource]) -> Result<(), Error> {

        let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;
        self.current_texture_id = Some(texture_id);
        self.current_texture_slot = None;

        self.gl.awsm_assign_texture_target_mips(bind_target, &opts, set_parameters, &srcs, &texture)
    }

    pub fn activate_texture_for_sampler_target(&mut self, bind_target:TextureTarget, texture_id: Id, sampler_name: &str) -> Result<(), Error> {
        let sampler_slot = {
            let program_id = self.current_program_id.ok_or(Error::from(NativeError::MissingShaderProgram))?;
            let program_info = self.program_lookup.get_mut(program_id).ok_or(Error::from(NativeError::MissingShaderProgram))?;

            let sampler_slot = program_info.texture_sampler_slot_lookup.get(sampler_name)
                .ok_or(Error::from(NativeError::MissingTextureSampler(Some(sampler_name.to_string()))))?;

            *sampler_slot
        };

        self.activate_texture_for_sampler_target_index(bind_target, texture_id, sampler_slot)?;
        Ok(())
    }

    pub fn activate_texture_for_sampler_target_index(&mut self, bind_target:TextureTarget, texture_id: Id, sampler_index: u32) -> Result<(), Error> {

        let entry = self.texture_sampler_lookup.get(sampler_index as usize).ok_or(Error::from(NativeError::Internal))?;

        let requires_activation = match entry {
            Some(entry) => {


                if entry.bind_target != bind_target || entry.texture_id != texture_id {
                    true
                } else {
                    false
                }
            },
            None => {
                true
            }
        };

        if requires_activation {
            self.texture_sampler_lookup[sampler_index as usize] = Some(TextureSamplerInfo{
                texture_id,
                bind_target
            });
            let texture = self.texture_lookup.get(texture_id).ok_or(Error::from(NativeError::MissingTexture))?;
            self.gl.awsm_activate_texture_for_sampler_target_index(bind_target, sampler_index, &texture);
        }

        Ok(())
    }
}