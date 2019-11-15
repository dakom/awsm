//these aren't worth putting behind features
pub mod errors;

//each of these can be enabled/disabled as needed
#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "canvas")]
pub mod canvas;
#[cfg(feature = "data")]
pub mod data;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "loaders")]
pub mod loaders;
#[cfg(feature = "tick")]
pub mod tick;
#[cfg(feature = "webgl")]
pub mod webgl;

#[cfg(feature = "window")]
pub mod window;

#[cfg(all(feature = "window", feature="workers"))]
pub mod global;