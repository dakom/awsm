use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Timestamp(pub f64);

#[derive(Serialize, Deserialize)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32 
}