
#[cfg(feature = "image")]
use super::image::Image;

use crate::data::TypedData;
use crate::data::*;
use crate::errors::{Error, NativeError};
use crate::window::get_window;
//Don't know why awsm_web needs FutureExt but awsm_renderer doesn't...
use futures::future::{self, TryFutureExt, FutureExt};
use std::future::Future;
use js_sys::{Array, ArrayBuffer, Promise};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Blob, BlobPropertyBag, Request, Response, Url,
};

#[cfg(feature = "image")]
use web_sys::{ HtmlImageElement };

#[cfg(feature = "audio")]
use web_sys::{ AudioBuffer, AudioContext };

#[cfg(feature = "image")]
pub fn image(url: &str) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    Image::new(url)
}

#[cfg(feature = "image")]
pub fn image_blob(blob: &Blob) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    match Url::create_object_url_with_blob(&blob) {
        Ok(url) => future::ok(url),
        Err(err) => future::err(err.into()),
    }
    .and_then(|url| image(&url))
}

#[cfg(feature = "image")]
pub fn image_u8<T: AsRef<[u8]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| image_blob(&blob))
}

#[cfg(feature = "image")]
pub fn image_u16<T: AsRef<[u16]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| image_blob(&blob))
}

#[cfg(feature = "image")]
pub fn image_u32<T: AsRef<[u32]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| image_blob(&blob))
}

#[cfg(feature = "image")]
pub fn image_i8<T: AsRef<[i8]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| image_blob(&blob))
}

#[cfg(feature = "image")]
pub fn image_i16<T: AsRef<[i16]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| image_blob(&blob))
}

#[cfg(feature = "image")]
pub fn image_i32<T: AsRef<[i32]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| image_blob(&blob))
}

#[cfg(feature = "image")]
pub fn image_f32<T: AsRef<[f32]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| image_blob(&blob))
}

#[cfg(feature = "image")]
pub fn image_f64<T: AsRef<[f64]>>(
    data: T,
    mime_type: &str,
) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    match Blob::new_with_buffer_source_sequence_and_options(
        &Array::of1(&TypedData::new(data.as_ref()).into()).into(),
        &blob_opts,
    ) {
        Ok(blob) => future::ok(blob),
        Err(err) => future::err(err.into()),
    }
    .and_then(|blob| image_blob(&blob))
}

//Audio

#[cfg(feature = "audio")]
pub fn audio<'a>(
    url: &str,
    ctx: &'a AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> + 'a {
    array_buffer(&url).and_then(move |buf| audio_buffer(&buf, &ctx))
}

#[cfg(feature = "audio")]
pub fn audio_buffer<'a>(
    array_buffer: &ArrayBuffer,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    match ctx.decode_audio_data(&array_buffer) {
        Ok(promise) => future::ok(promise),
        Err(err) => future::err(err.into()),
    }
    .and_then(|promise| JsFuture::from(promise))
    .map(|res| match res {
        Ok(x) => Ok(AudioBuffer::from(x)),
        Err(x) => Err(Error::from(x)),
    })
}

//convenince helpers for loading slices, vecs, etc.
#[cfg(feature = "audio")]
pub fn audio_u8<T: AsRef<[u8]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}

#[cfg(feature = "audio")]
pub fn audio_u16<T: AsRef<[u16]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
#[cfg(feature = "audio")]
pub fn audio_u32<T: AsRef<[u32]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
#[cfg(feature = "audio")]
pub fn audio_i8<T: AsRef<[i8]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
#[cfg(feature = "audio")]
pub fn audio_i16<T: AsRef<[i16]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
#[cfg(feature = "audio")]
pub fn audio_i32<T: AsRef<[i32]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
#[cfg(feature = "audio")]
pub fn audio_f32<T: AsRef<[f32]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}
#[cfg(feature = "audio")]
pub fn audio_f64<T: AsRef<[f64]>>(
    data: T,
    ctx: &AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> {
    let array_buffer: ArrayBuffer = TypedData::new(data.as_ref()).into();
    audio_buffer(&array_buffer, &ctx)
}

//text
pub fn text(url: &str) -> impl Future<Output = Result<String, Error>> {
    let req = Request::new_with_str(url);

    async {
        let req = req?;

        let resp: Response = request(&req).await?;

        let promise = resp.text()?;

        let data = JsFuture::from(promise).await?;

        let text = data.as_string().ok_or(Error::from(NativeError::Internal))?;

        Ok(text)
    }
}

//pure data
pub fn array_buffer(url: &str) -> impl Future<Output = Result<ArrayBuffer, Error>> {
    let req = Request::new_with_str(url);

    async {
        let req = req?;

        let resp: Response = request(&req).await?;

        let promise = resp.array_buffer()?;

        let data = JsFuture::from(promise).await?;

        Ok(data.into())
    }
}

pub fn vec_f32(url: &str) -> impl Future<Output = Result<Vec<f32>, Error>> {
    let url = url.to_owned();
    async move {
        let data = array_buffer(&url).await?;
        let data: ArrayBuffer = data.into();
        Ok(clone_to_vec_f32(&js_sys::Float32Array::new(&data)))
    }
}

pub fn vec_f64(url: &str) -> impl Future<Output = Result<Vec<f64>, Error>> {
    let url = url.to_owned();
    async move {
        let data = array_buffer(&url).await?;
        let data: ArrayBuffer = data.into();
        Ok(clone_to_vec_f64(&js_sys::Float64Array::new(&data)))
    }
}

pub fn vec_i8(url: &str) -> impl Future<Output = Result<Vec<i8>, Error>> {
    let url = url.to_owned();
    async move {
        let data = array_buffer(&url).await?;
        let data: ArrayBuffer = data.into();
        Ok(clone_to_vec_i8(&js_sys::Int8Array::new(&data)))
    }
}
pub fn vec_i16(url: &str) -> impl Future<Output = Result<Vec<i16>, Error>> {
    let url = url.to_owned();
    async move {
        let data = array_buffer(&url).await?;
        let data: ArrayBuffer = data.into();
        Ok(clone_to_vec_i16(&js_sys::Int16Array::new(&data)))
    }
}
pub fn vec_i32(url: &str) -> impl Future<Output = Result<Vec<i32>, Error>> {
    let url = url.to_owned();
    async move {
        let data = array_buffer(&url).await?;
        let data: ArrayBuffer = data.into();
        Ok(clone_to_vec_i32(&js_sys::Int32Array::new(&data)))
    }
}

pub fn vec_u8(url: &str) -> impl Future<Output = Result<Vec<u8>, Error>> {
    let url = url.to_owned();
    async move {
        let data = array_buffer(&url).await?;
        let data: ArrayBuffer = data.into();
        Ok(clone_to_vec_u8(&js_sys::Uint8Array::new(&data)))
    }
}
pub fn vec_u16(url: &str) -> impl Future<Output = Result<Vec<u16>, Error>> {
    let url = url.to_owned();
    async move {
        let data = array_buffer(&url).await?;
        let data: ArrayBuffer = data.into();
        Ok(clone_to_vec_u16(&js_sys::Uint16Array::new(&data)))
    }
}
pub fn vec_u32(url: &str) -> impl Future<Output = Result<Vec<u32>, Error>> {
    let url = url.to_owned();
    async move {
        let data = array_buffer(&url).await?;
        let data: ArrayBuffer = data.into();
        Ok(clone_to_vec_u32(&js_sys::Uint32Array::new(&data)))
    }
}

pub fn request(req: &Request) -> impl Future<Output = Result<Response, Error>> {
    let promise: Result<Promise, Error> =
        get_window().map(|window| window.fetch_with_request(&req));

    async {
        let promise = promise?;

        let resp_value = JsFuture::from(promise).await?;
        let resp: Response = resp_value.dyn_into()?;

        Ok(resp)
    }
}
