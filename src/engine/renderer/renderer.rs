use crate::errors::{Error};
use specs::{World, WorldExt};
use web_sys::{
    Window,
    HtmlCanvasElement,
};
use awsm::webgl::{
    ClearBufferMask,
    WebGl2Renderer,
    WebGlContextOptions,
    get_webgl_context_2,
    Id,
};

pub fn setup_renderer(
    world:&mut World,
    canvas: &HtmlCanvasElement, 
    opts: Option<&WebGlContextOptions>,
    clear_color:Option<(f32, f32, f32, f32)>,
) -> Result<(), Error> {

    let gl = get_webgl_context_2(canvas, opts)?;
    let webgl = WebGl2Renderer::new(gl)?;

    if let Some((r,g,b,a)) = clear_color {
        webgl.gl.clear_color(r,g,b,a);
    }

    Ok(())
}
/*    

    pub fn resize(&mut self, width: u32, height: u32) {
        self.webgl.resize(width, height);
    }

    pub fn render(&mut self) {
        self.webgl.clear(&[
            ClearBufferMask::ColorBufferBit,
            ClearBufferMask::DepthBufferBit,
        ]);
    }
    */
