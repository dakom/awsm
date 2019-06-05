use web_sys::{Window};

pub fn get_size(window:&web_sys::Window) -> Result<(u32, u32), &'static str> {
    /*
    let document_element = 
        window
            .document()
            .and_then(|doc| doc.document_element())
            .ok_or("should have document")?;

    let width = document_element.client_width();
    let height = document_element.client_height();
    */
    let width =
        window
            .inner_width()
            .ok()
            .and_then(|val| val.as_f64())
            .ok_or("couldn't get window width")?;

    let height = 
        window
            .inner_height()
            .ok()
            .and_then(|val| val.as_f64())
            .ok_or("couldn't get window height")?;

    Ok((width as u32, height as u32))
}
