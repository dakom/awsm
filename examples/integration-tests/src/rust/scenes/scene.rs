use awsm_webgl::errors::*;

pub trait Scene {
    fn id(&self) -> &str;
    fn tick(&mut self, time_stamp:f64, delta_time:f64) -> Result<(), Error>;
    fn resize(&mut self, width: u32, height: u32) -> Result<(), Error>;
}
