#![feature(async_await, await_macro)]

//these are used in enough places that
//it makes no sense to put them behind features
pub mod errors;

//each of these can be enabled/disabled as needed
#[cfg(feature = "data")]
pub mod data;
#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "canvas")]
pub mod canvas;
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
