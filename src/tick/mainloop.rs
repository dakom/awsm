/// A Rust port of https://github.com/IceCreamYou/MainLoop.js
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
use web_sys::{Window};
use log::{info};
use crate::errors::{Error};
use crate::window::{get_window};
use super::{request_animation_frame, start_raf_loop};

/// A fixed-timestep game loop. 
/// Essentially, a Rust port of https://github.com/IceCreamYou/MainLoop.js
pub struct MainLoop <B,U,D,E>
    where
        B: Fn(f64, f64) -> (),
        U: Fn(f64) -> (),
        D: Fn(f64) -> (),
        E: Fn(f64, bool) -> (),
{
    /// The amount of time (in milliseconds) to simulate each time update()
    /// runs. See `MainLoop.setSimulationTimestep()` for details.
    pub simulation_timestep:f64,

    /// The cumulative amount of in-app time that hasn't been simulated yet.
    /// See the comments inside animate() for details.
    pub frame_delta:f64,

    /// The timestamp in milliseconds of the last time the main loop was run.
    /// Used to compute the time elapsed between frames.
    pub last_frame_time_ms:f64,

    /// An exponential moving average of the frames per second.
    pub fps:u32,

    /// A factor that affects how heavily to weight more recent seconds'
    /// performance when calculating the average frames per second. Valid values
    /// range from zero to one inclusive. Higher values result in weighting more
    /// recent seconds more heavily.
    pub fps_alpha: f64,

    /// The minimum duration between updates to the frames-per-second estimate.
    /// Higher values increase accuracy, but result in slower updates.
    pub fps_update_interval: f64,

    /// The timestamp (in milliseconds) of the last time the `fps` moving
    /// average was updated.
    pub last_fps_update: f64,

    /// The number of frames delivered since the last time the `fps` moving
    /// average was updated (i.e. since `lastFpsUpdate`).
    pub frames_since_last_fps_update: u32,

    /// The number of times update() is called in a given frame. This is only
    /// relevant inside of animate(), but a reference is held externally so that
    /// this variable is not marked for garbage collection every time the main
    /// loop runs.
    pub num_update_steps: u32,

    /// The minimum amount of time in milliseconds that must pass since the last
    /// frame was executed before another frame can be executed. The
    /// multiplicative inverse caps the FPS (the default of zero means there is
    /// no cap).
    pub min_frame_delay: f64,

    /// Whether the main loop is running.
    pub running: bool,

    /// `true` if `MainLoop.start()` has been called and the most recent time it
    /// was called has not been followed by a call to `MainLoop.stop()`. This is
    /// different than `running` because there is a delay of a few milliseconds
    /// after `MainLoop.start()` is called before the application is considered
    /// "running." This delay is due to waiting for the next frame.
    pub started: bool,

    /// Whether the simulation has fallen too far behind real time.
    /// Specifically, `panic` will be set to `true` if too many updates occur in
    /// one frame. This is only relevant inside of animate(), but a reference is
    /// held externally so that this variable is not marked for garbage
    /// collection every time the main loop runs.
    pub panic: bool,

    /// A function that runs at the beginning of the main loop.
    /* The begin() function is typically used to process input before the
     * updates run. Processing input here (in chunks) can reduce the running
     * time of event handlers, which is useful because long-running event
     * handlers can sometimes delay frames.
     *
     * Unlike update(), which can run zero or more times per
     * frame, begin() always runs exactly once per frame. This makes it useful
     * for any updates that are not dependent on time in the simulation.
     * Examples include adjusting HUD calculations or performing long-running
     * updates incrementally. Compared to end(), generally
     * actions should occur in begin() if they affect anything that
     * update() or draw() use 
     *
     * * timestamp 
     *   The current timestamp (when the frame started), in milliseconds. This
     *   should only be used for comparison to other timestamps because the
     *   epoch (i.e. the "zero" time) depends on the engine running this code.
     *   In engines that support `DOMHighResTimeStamp` (all modern browsers
     *   except iOS Safari 8) the epoch is the time the page started loading,
     *   specifically `performance.timing.navigationStart`. Everywhere else,
     *   including node.js, the epoch is the Unix epoch (1970-01-01T00:00:00Z).
     * * delta
     *   The total elapsed time that has not yet been simulated, in
     *   milliseconds.
     */
    pub begin: B,
    
    /// The function that runs updates (e.g. AI and physics).
    /**
     *
     * The update() function should simulate anything that is affected by time.
     * It can be called zero or more times per frame depending on the frame
     * rate.
     *
     * As with everything in the main loop, the running time of update()
     * directly affects the frame rate. If update() takes long enough that the
     * frame rate drops below the target ("budgeted") frame rate, parts of the
     * update() function that do not need to execute between every frame can be
     * moved into Web Workers. (Various sources on the internet sometimes
     * suggest other scheduling patterns using setTimeout() or setInterval().
     * These approaches sometimes offer modest improvements with minimal
     * changes to existing code, but because JavaScript is single-threaded, the
     * updates will still block rendering and drag down the frame rate. Web
     * Workers execute in separate threads, so they free up more time in the
     * main loop.)
     *
     * This script can be imported into a Web Worker using importScripts() and
     * used to run a second main loop in the worker. Some considerations:
     *
     * - Profile your code before doing the work to move it into Web Workers.
     *   It could be the rendering that is the bottleneck, in which case the
     *   solution is to decrease the visual complexity of the scene.
     * - It doesn't make sense to move the *entire* contents of update() into
     *   workers unless {@link #setDraw draw}() can interpolate between frames.
     *   The lowest-hanging fruit is background updates (like calculating
     *   citizens' happiness in a city-building game), physics that doesn't
     *   affect the scene (like flags waving in the wind), and anything that is
     *   occluded or happening far off screen.
     * - If draw() needs to interpolate physics based on activity that occurs
     *   in a worker, the worker needs to pass the interpolation value back to
     *   the main thread so that is is available to draw().
     * - Web Workers can't access the state of the main thread, so they can't
     *   directly modify objects in your scene. Moving data to and from Web
     *   Workers is a pain. The fastest way to do it is with Transferable
     *   Objects: basically, you can pass an ArrayBuffer to a worker,
     *   destroying the original reference in the process.
     *
     * You can read more about Web Workers and Transferable Objects at
     * [HTML5 Rocks](http://www.html5rocks.com/en/tutorials/workers/basics/).
     *
     * @param {Function} update
     *   The update() function.
     * @param {Number} [update.delta]
     *   The amount of time in milliseconds to simulate in the update. In most
     *   cases this timestep never changes in order to ensure deterministic
     *   updates. The timestep is the same as that returned by
     *   `MainLoop.getSimulationTimestep()`.
     */
    pub update: U,

    /// A function that draws things on the screen.
    /*
     * The draw() function gets passed the percent of time that the next run of
     * {@link #setUpdate update}() will simulate that has actually elapsed, as
     * a decimal. In other words, draw() gets passed how far between update()
     * calls it is. This is useful because the time simulated by update() and
     * the time between draw() calls is usually different, so the parameter to
     * draw() can be used to interpolate motion between frames to make
     * rendering appear smoother. To illustrate, if update() advances the
     * simulation at each vertical bar in the first row below, and draw() calls
     * happen at each vertical bar in the second row below, then some frames
     * will have time left over that is not yet simulated by update() when
     * rendering occurs in draw():
     *
     *     update() timesteps:  |  |  |  |  |  |  |  |  |
     *     draw() calls:        |   |   |   |   |   |   |
     *
     * To interpolate motion for rendering purposes, objects' state after the
     * last update() must be retained and used to calculate an intermediate
     * state. Note that this means renders will be up to one update() behind.
     * This is still better than extrapolating (projecting objects' state after
     * a future update()) which can produce bizarre results. Storing multiple
     * states can be difficult to set up, and keep in mind that running this
     * process takes time that could push the frame rate down, so it's often
     * not worthwhile unless stuttering is visible.
     *
     * @param {Function} draw
     *   The draw() function.
     * @param {Number} [draw.interpolationPercentage]
     *   The cumulative amount of time that hasn't been simulated yet, divided
     *   by the amount of time that will be simulated the next time update()
     *   runs. Useful for interpolating frames.
     */
    pub draw: D,

    /// A function that runs at the end of the main loop.
    /*
     * Unlike {@link #setUpdate update}(), which can run zero or more times per
     * frame, end() always runs exactly once per frame. This makes it useful
     * for any updates that are not dependent on time in the simulation.
     * Examples include cleaning up any temporary state set up by
     * {@link #setBegin begin}(), lowering the visual quality if the frame rate
     * is too low, or performing long-running updates incrementally. Compared
     * to begin(), generally actions should occur in end() if they use anything
     * that update() or {@link #setDraw draw}() affect.
     *
     * @param {Function} end
     *   The end() function.
     * @param {Number} [end.fps]
     *   The exponential moving average of the frames per second. This is the
     *   same value returned by `MainLoop.getFPS()`. It can be used to take
     *   action when the FPS is too low (or to restore to normalcy if the FPS
     *   moves back up). Examples of actions to take if the FPS is too low
     *   include exiting the application, lowering the visual quality, stopping
     *   or reducing activities outside of the main loop like event handlers or
     *   audio playback, performing non-critical updates less frequently, or
     *   increasing the simulation timestep (by calling
     *   `MainLoop.setSimulationTimestep()`). Note that this last option
     *   results in more time being simulated per update() call, which causes
     *   the application to behave non-deterministically.
     * @param {Boolean} [end.panic=false]
     *   Indicates whether the simulation has fallen too far behind real time.
     *   Specifically, `panic` will be `true` if too many updates occurred in
     *   one frame. In networked lockstep applications, the application should
     *   wait for some amount of time to see if the user can catch up before
     *   dropping the user. In networked but non-lockstep applications, this
     *   typically indicates that the user needs to be snapped or eased to the
     *   current authoritative state. When this happens, it may be convenient
     *   to call `MainLoop.resetFrameDelta()` to discard accumulated pending
     *   updates. In non-networked applications, it may be acceptable to allow
     *   the application to keep running for awhile to see if it will catch up.
     *   However, this could also cause the application to look like it is
     *   running very quickly for a few frames as it transitions through the
     *   intermediate states. An alternative that may be acceptable is to
     *   simply ignore the unsimulated elapsed time by calling
     *   `MainLoop.resetFrameDelta()` even though this introduces
     *   non-deterministic behavior. In all cases, if the application panics
     *   frequently, this is an indication that the main loop is running too
     *   slowly. However, most of the time the drop in frame rate will probably
     *   be noticeable before a panic occurs. To help the application catch up
     *   after a panic caused by a spiral of death, the same steps can be taken
     *   that are suggested above if the FPS drops too low.
     */
    pub end: E,

    /// Cancel function for stopping the loop 
    cancel_fn: Option<Box<dyn FnOnce() -> ()>>
}

impl <B,U,D,E> MainLoop <B,U,D,E> 
    where
        B: Fn(f64, f64) -> (),
        U: Fn(f64) -> (),
        D: Fn(f64) -> (),
        E: Fn(f64, bool) -> (),
{

    pub fn new(begin: B, update: U, draw: D, end: E) -> Self {
        Self {
            simulation_timestep: 1000.0 / 60.0,
            frame_delta: 0.0,
            last_frame_time_ms: 0.0,
            fps: 60,
            fps_alpha: 0.9,
            fps_update_interval: 1000.0,
            last_fps_update: 0.0,
            frames_since_last_fps_update: 0,
            num_update_steps: 0,
            min_frame_delay: 0.0,
            running: false,
            started: false,
            panic: false,
            begin,
            update,
            draw,
            end,
            cancel_fn: None
        }
    }

    /**
     * Gets the maximum frame rate.
     *
     * Other factors also limit the FPS; see [set_simulation_timestep](set_simulation_timestep)`
     * for details.
     *
     * See also `[set_max_allowed_fps](set_max_allowed_fps)`.
     *
     * returns the maximum number of frames per second allowed.
     */
    pub fn get_max_allowed_fps(&self) -> u32 {
        (1000.0 / self.min_frame_delay).round() as u32
    }

    /**
     * Sets a maximum frame rate.
     *
     * See also [get_max_allowed_fps](get_max_allowed_fps).
     *
     * fps: The maximum number of frames per second to execute. If None,
     *   there will be no FPS cap (although other factors do limit the
     *   FPS; see [set_simulation_timestep](set_simulation_timestep) for details). 
     *   If zero, this will stop the loop, and when the loop is next started, 
     *   it will return to the previous maximum frame rate. Passing negative 
     *   values will stall the loop until this function is called again with a positive value.
     *
     */
    pub fn set_max_allowed_fps(&mut self, fps:Option<i32>) {
        match fps {
            None => self.min_frame_delay = 0.0,
            Some(fps) => {
                if fps == 0 {
                    self.stop();
                } else {
                    self.min_frame_delay = 1000.0 / (fps as f64)
                }
            }
        }
    }

    /**
     * Reset the amount of time that has not yet been simulated to zero.
     *
     * This introduces non-deterministic behavior if called after the
     * application has started running (unless it is being reset, in which case
     * it doesn't matter). However, this can be useful in cases where the
     * amount of time that has not yet been simulated has grown very large
     * (for example, when the application's tab gets put in the background and
     * the browser throttles the timers as a result). In applications with
     * lockstep the player would get dropped, but in other networked
     * applications it may be necessary to snap or ease the player/user to the
     * authoritative state and discard pending updates in the process. In
     * non-networked applications it may also be acceptable to simply resume
     * the application where it last left off and ignore the accumulated
     * unsimulated time.
     *
     * @return {Number}
     *   The cumulative amount of elapsed time in milliseconds that has not yet
     *   been simulated, but is being discarded as a result of calling this
     *   function.
     */
    pub fn reset_frame_delta(&mut self) -> f64 {
        let old_frame_delta = self.frame_delta;
        self.frame_delta = 0.0;
        old_frame_delta
    }


    /**
     * Starts the main loop.
     *
     * Note that the application is not considered "running" immediately after
     * this function returns; rather, it is considered "running" after the
     * application draws its first frame. The distinction is that event
     * handlers should remain paused until the application is running, even
     * after `MainLoop.start()` is called. Check `MainLoop.isRunning()` for the
     * current status. To act after the application starts, register a callback
     * with requestAnimationFrame() after calling this function and execute the
     * action in that callback. It is safe to call `MainLoop.start()` multiple
     * times even before the application starts running and without calling
     * `MainLoop.stop()` in between, although there is no reason to do this;
     * the main loop will only start if it is not already started.
     *
     * See also `MainLoop.stop()`.
     */

    pub fn start(&mut self) -> Result<(), Error> {
        if !self.started {
            self.started = true;

            let window = get_window()?; 

            let mut cb = Closure::wrap(Box::new(move |time| {
            }) as Box<dyn FnMut(f64) -> ());

            let raf_id = request_animation_frame(&window, &cb).ok();
            /*
            let cancel_fn = start_raf_loop(|time| {
            })?;

            self.cancel_fn = Some(Box::new(cancel_fn));
            */
        }

        Ok(())
    }

    pub fn stop(&mut self) {
    }
}
