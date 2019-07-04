use crate::errors::{Error, NativeError};
use futures::{Future};
use wasm_bindgen::{JsValue};
use wasm_bindgen::JsCast;
use js_sys::{Promise};
use web_sys::{Request, Response};
use crate::window::{get_window};
use wasm_bindgen_futures::futures_0_3::{JsFuture};

pub fn text_url(url:&str) -> impl Future<Output= Result<String, Error>> { 
    let req = Request::new_with_str(url);

    async {
        let req = req?;

        let resp:Response = request(&req).await?;

        let promise = resp.text()?;

        let text_value = JsFuture::from(promise).await?;

        let text = text_value.as_string().ok_or(Error::from(NativeError::Internal))?;

        Ok(text)
    }
}


pub fn request(req:&Request) -> impl Future<Output= Result<Response, Error>> { 
    let promise:Result<Promise, Error> = 
        get_window().map(|window| window.fetch_with_request(&req));

    async {
        let promise = promise?;

        let resp_value = JsFuture::from(promise).await?;
        let resp:Response = resp_value.dyn_into()?;

        Ok(resp)
    }
}
