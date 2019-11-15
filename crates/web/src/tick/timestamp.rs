use super::RafLoop;
use crate::errors::Error;
///Simple struct for time, deltatime, and elapsed time
#[derive(Copy, Clone, Debug)]
pub struct Timestamp {
    /// the current time
    pub time: f64,
    /// change in time since last tick
    pub delta: f64,
    /// total elapsed time since loop started
    pub elapsed: f64,
}

pub struct TimestampLoop {
    pub raf_loop: RafLoop,
}

impl TimestampLoop {
    /// similar to the top-level start_raf_loop() but instead of a callback with the current time
    /// it provides a Timestamp struct which contains commonly useful info
    pub fn start<F>(mut on_tick: F) -> Result<Self, Error>
    where
        F: (FnMut(Timestamp) -> ()) + 'static,
    {
        let mut last_time: Option<f64> = None;
        let mut first_time = 0f64;

        let raf_loop = RafLoop::start(move |time| {
            match last_time {
                Some(last_time) => {
                    on_tick(Timestamp {
                        time,
                        delta: time - last_time,
                        elapsed: time - first_time,
                    });
                }
                None => {
                    on_tick(Timestamp {
                        time,
                        delta: 0.0,
                        elapsed: 0.0,
                    });
                    first_time = time;
                }
            }
            last_time = Some(time);
        })?;

        Ok(Self { raf_loop })
    }
}