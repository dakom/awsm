use crate::errors::{Error, NativeError};
use web_sys::Window;

pub fn get_window_size(window: &web_sys::Window) -> Result<(u32, u32), Error> {
    /*
    let document_element =
        window
            .document()
            .and_then(|doc| doc.document_element())
            .ok_or("should have document")?;

    let width = document_element.client_width();
    let height = document_element.client_height();
    */
    let width = window
        .inner_width()
        .ok()
        .and_then(|val| val.as_f64())
        .ok_or(Error::Native(NativeError::WindowWidth))?;

    let height = window
        .inner_height()
        .ok()
        .and_then(|val| val.as_f64())
        .ok_or(Error::Native(NativeError::WindowHeight))?;

    Ok((width as u32, height as u32))
}

pub fn get_window() -> Result<Window, Error> {
    web_sys::window().ok_or(Error::Native(NativeError::Window))
}
