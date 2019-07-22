use super::image::Image;
use crate::errors::{Error, NativeError};
use crate::window::get_window;
use futures::Future;
use js_sys::{ArrayBuffer, Promise};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::futures_0_3::JsFuture;
use web_sys::{AudioBuffer, AudioContext, HtmlImageElement, Request, Response};

pub fn image(url: &str) -> impl Future<Output = Result<HtmlImageElement, Error>> {
    Image::new(url)
}

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

pub fn audio_buffer<'a>(
    url: &str,
    ctx: &'a AudioContext,
) -> impl Future<Output = Result<AudioBuffer, Error>> + 'a {
    let url = url.to_owned();

    async move {
        let audio_data = array_buffer(&url).await?;

        let promise = ctx.decode_audio_data(&audio_data)?;

        let data = JsFuture::from(promise).await?;

        Ok(data.into())
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
