use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use std::convert::TryFrom;

//the order must match typescript!
#[derive(FromPrimitive)]
#[repr(u32)]
pub enum IoEventIndex {
    LoopBegin,
    LoopUpdate,
    LoopDraw,
    LoopEnd,
    WindowSize,
    LoadGltf
}

//Let's us get a IoEvent from the number which is sent from JS
impl TryFrom<u32> for IoEventIndex {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or("IoEvent: outside of range!")
    }
}

