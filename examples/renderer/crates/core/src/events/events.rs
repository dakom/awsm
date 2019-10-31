use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

//the order must match typescript!
#[derive(FromPrimitive)]
#[repr(u32)]
pub enum BridgeEventIndex {
    WindowSize,
    LoadGltf,
    GltfLoaded,
}

//Let's us get a BridgeEvent from the number which is sent from JS
impl TryFrom<u32> for BridgeEventIndex {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("BridgeEvent: outside of range!")
    }
}

//All the event data:

#[derive(Serialize, Deserialize)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32 
}