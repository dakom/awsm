///Options for start_main_loop()
pub struct MainLoopOptions {
    /// The amount of time (in milliseconds) to simulate each time update()
    /// runs. See `MainLoop.setSimulationTimestep()` for details.
    pub simulation_timestep: f64,

    /// A factor that affects how heavily to weight more recent seconds'
    /// performance when calculating the average frames per second. Valid values
    /// range from zero to one inclusive. Higher values result in weighting more
    /// recent seconds more heavily.
    pub fps_alpha: f64,

    /// The minimum duration between updates to the frames-per-second estimate.
    /// Higher values increase accuracy, but result in slower updates.
    pub fps_update_interval: f64,

    /// The minimum amount of time in milliseconds that must pass since the last
    /// frame was executed before another frame can be executed. The
    /// multiplicative inverse caps the FPS (the default of zero means there is
    /// no cap).
    pub min_frame_delay: f64,
}

impl Default for MainLoopOptions {
    fn default() -> Self {
        Self {
            simulation_timestep: 1000.0 / 60.0,
            fps_alpha: 0.9,
            fps_update_interval: 1000.0,
            min_frame_delay: 0.0,
        }
    }
}

/// A Rust port of https://github.com/IceCreamYou/MainLoop.js
///
/// It's pretty much a direct port, except for two differences:
/// 1. it all runs in one loop with a branch (probably cheaper than passing the required Rc/RefCells around)
/// 2. starting/stopping is explicit via cancelling and restarting (there is no reset_frame_delta() or runtime fps cap)
///
///
/// @begin: A function that runs at the beginning of the main loop.
///  
/// The begin() function is typically used to process input before the
/// updates run. Processing input here (in chunks) can reduce the running
/// time of event handlers, which is useful because long-running event
/// handlers can sometimes delay frames.
///
/// Unlike update(), which can run zero or more times per
/// frame, begin() always runs exactly once per frame. This makes it useful
/// for any updates that are not dependent on time in the simulation.
/// Examples include adjusting HUD calculations or performing long-running
/// updates incrementally. Compared to end(), generally
/// actions should occur in begin() if they affect anything that
/// update() or draw() use
///
/// * timestamp
///
///   The current timestamp (when the frame started), in milliseconds. This
///   should only be used for comparison to other timestamps because the
///   epoch (i.e. the "zero" time) depends on the engine running this code.
///   In engines that support `DOMHighResTimeStamp` (all modern browsers
///   except iOS Safari 8) the epoch is the time the page started loading,
///   specifically `performance.timing.navigationStart`. Everywhere else,
///   including node.js, the epoch is the Unix epoch (1970-01-01T00:00:00Z).
///
/// * delta
///
///   The total elapsed time that has not yet been simulated, in
///   milliseconds.
///
/// @update: The function that runs updates (e.g. AI and physics).
///
/// The update() function should simulate anything that is affected by time.
/// It can be called zero or more times per frame depending on the frame
/// rate.
///
/// As with everything in the main loop, the running time of update()
/// directly affects the frame rate. If update() takes long enough that the
/// frame rate drops below the target ("budgeted") frame rate, parts of the
/// update() function that do not need to execute between every frame can be
/// moved into Web Workers. (Various sources on the internet sometimes
/// suggest other scheduling patterns using setTimeout() or setInterval().
/// These approaches sometimes offer modest improvements with minimal
/// changes to existing code, but because JavaScript is single-threaded, the
/// updates will still block rendering and drag down the frame rate. Web
/// Workers execute in separate threads, so they free up more time in the
/// main loop.)
///
/// This script can be imported into a Web Worker using importScripts() and
/// used to run a second main loop in the worker. Some considerations:
///
/// - Profile your code before doing the work to move it into Web Workers.
///   It could be the rendering that is the bottleneck, in which case the
///   solution is to decrease the visual complexity of the scene.
/// - It doesn't make sense to move the *entire* contents of update() into
///   workers unless draw() can interpolate between frames.
///   The lowest-hanging fruit is background updates (like calculating
///   citizens' happiness in a city-building game), physics that doesn't
///   affect the scene (like flags waving in the wind), and anything that is
///   occluded or happening far off screen.
/// - If draw() needs to interpolate physics based on activity that occurs
///   in a worker, the worker needs to pass the interpolation value back to
///   the main thread so that is is available to draw().
/// - Web Workers can't access the state of the main thread, so they can't
///   directly modify objects in your scene. Moving data to and from Web
///   Workers is a pain. The fastest way to do it is with Transferable
///   Objects: basically, you can pass an ArrayBuffer to a worker,
///   destroying the original reference in the process.
///
/// You can read more about Web Workers and Transferable Objects at
/// [HTML5 Rocks](http://www.html5rocks.com/en/tutorials/workers/basics/).
///
/// * delta
///
///   The amount of time in milliseconds to simulate in the update. In most
///   cases this timestep never changes in order to ensure deterministic
///   updates. The timestep is the same as that returned by
///   `MainLoop.getSimulationTimestep()`.
///
/// @draw: A function that draws things on the screen.
///
/// The draw() function gets passed the percent of time that the next run of
/// update() will simulate that has actually elapsed, as
/// a decimal. In other words, draw() gets passed how far between update()
/// calls it is. This is useful because the time simulated by update() and
/// the time between draw() calls is usually different, so the parameter to
/// draw() can be used to interpolate motion between frames to make
/// rendering appear smoother. To illustrate, if update() advances the
/// simulation at each vertical bar in the first row below, and draw() calls
/// happen at each vertical bar in the second row below, then some frames
/// will have time left over that is not yet simulated by update() when
/// rendering occurs in draw():
///
/// update() timesteps:  |  |  |  |  |  |  |  |  |
/// draw() calls:        |   |   |   |   |   |   |
///
/// To interpolate motion for rendering purposes, objects' state after the
/// last update() must be retained and used to calculate an intermediate
/// state. Note that this means renders will be up to one update() behind.
/// This is still better than extrapolating (projecting objects' state after
/// a future update()) which can produce bizarre results. Storing multiple
/// states can be difficult to set up, and keep in mind that running this
/// process takes time that could push the frame rate down, so it's often
/// not worthwhile unless stuttering is visible.
///
/// * interpolation_percentage
///
///   The cumulative amount of time that hasn't been simulated yet, divided
///   by the amount of time that will be simulated the next time update()
///   runs. Useful for interpolating frames.
///
/// @end: A function that runs at the end of the main loop.
///
/// Unlike update(), which can run zero or more times per
/// frame, end() always runs exactly once per frame. This makes it useful
/// for any updates that are not dependent on time in the simulation.
/// Examples include cleaning up any temporary state set up by
/// begin(), lowering the visual quality if the frame rate
/// is too low, or performing long-running updates incrementally. Compared
/// to begin(), generally actions should occur in end() if they use anything
/// that update() or draw() affect.
///
/// * fps
///
///   The exponential moving average of the frames per second. It can be used
///   to take action when the FPS is too low (or to restore to normalcy if the FPS
///   moves back up). Examples of actions to take if the FPS is too low
///   include exiting the application, lowering the visual quality, stopping
///   or reducing activities outside of the main loop like event handlers or
///   audio playback, performing non-critical updates less frequently, or
///   restarting with a higher simulation timestep.  Note that this last option
///   results in more time being simulated per update() call, which causes
///   the application to behave non-deterministically.
///
/// * end_panic
///
///   Indicates whether the simulation has fallen too far behind real time.
///   Specifically, `panic` will be `true` if too many updates occurred in
///   one frame. In networked lockstep applications, the application should
///   wait for some amount of time to see if the user can catch up before
///   dropping the user. In networked but non-lockstep applications, this
///   typically indicates that the user needs to be snapped or eased to the
///   current authoritative state. When this happens, it may be convenient
///   to call `MainLoop.resetFrameDelta()` to discard accumulated pending
///   updates. In non-networked applications, it may be acceptable to allow
///   the application to keep running for awhile to see if it will catch up.
///   However, this could also cause the application to look like it is
///   running very quickly for a few frames as it transitions through the
///   intermediate states. If the application panics
///   frequently, this is an indication that the main loop is running too
///   slowly. However, most of the time the drop in frame rate will probably
///   be noticeable before a panic occurs. To help the application catch up
///   after a panic caused by a spiral of death, the same steps can be taken
///   that are suggested above if the FPS drops too low.
pub struct MainLoop <B,U,D,E> 
where
    B: FnMut(f64, f64) -> () + 'static,
    U: FnMut(f64) -> () + 'static,
    D: FnMut(f64) -> () + 'static,
    E: FnMut(f64, bool) -> () + 'static,
{
    begin: B,
    update: U,
    draw: D,
    end: E,

    /// user-configurable options
    opts: MainLoopOptions,

    /// Whether the main loop is running.
    running: bool,

    /// The cumulative amount of in-app time that hasn't been simulated yet.
    /// See the comments inside animate() for details.
    frame_delta: f64,

    /// The timestamp in milliseconds of the last time the main loop was run.
    /// Used to compute the time elapsed between frames.
    last_frame_time_ms: f64,

    /// An exponential moving average of the frames per second.
    fps: f64,

    /// The timestamp (in milliseconds) of the last time the `fps` moving
    /// average was updated.
    last_fps_update: f64,

    /// The number of frames delivered since the last time the `fps` moving
    /// average was updated (i.e. since `lastFpsUpdate`).
    frames_since_last_fps_update: u32,

    /// The number of times update() is called in a given frame. This is only
    /// relevant inside of animate(), but a reference is held externally so that
    /// this variable is not marked for garbage collection every time the main
    /// loop runs.
    num_update_steps: u32,

    /// Whether the simulation has fallen too far behind real time.
    /// Specifically, `panic` will be set to `true` if too many updates occur in
    /// one frame. This is only relevant inside of animate(), but a reference is
    /// held externally so that this variable is not marked for garbage
    /// collection every time the main loop runs.
    end_panic: bool,
}


impl <B,U,D,E> Drop for MainLoop<B,U,D,E> 
where
    B: FnMut(f64, f64) -> () + 'static,
    U: FnMut(f64) -> () + 'static,
    D: FnMut(f64) -> () + 'static,
    E: FnMut(f64, bool) -> () + 'static,
{
    fn drop(&mut self) {
        #[cfg(feature = "debug_log")]
        log::info!("Main Loop Dropped (but not whatever was passed into the closure!)");
    }
}

impl <B,U,D,E> MainLoop <B,U,D,E> 
where
    B: FnMut(f64, f64) -> () + 'static,
    U: FnMut(f64) -> () + 'static,
    D: FnMut(f64) -> () + 'static,
    E: FnMut(f64, bool) -> () + 'static,
{
    pub fn new(opts: MainLoopOptions, begin: B, update: U, draw: D, end: E) -> Self 
    {
        Self{
            begin,
            update,
            draw,
            end,
            opts,
            running: false,
            frame_delta: 0.0,
            last_frame_time_ms: 0.0,
            fps: 60.0,
            last_fps_update: 0.0,
            frames_since_last_fps_update: 0,
            num_update_steps: 0,
            end_panic: false
        } 

    }

    pub fn tick(&mut self, timestamp: f64) {
            if !self.running {
                // Render the initial state before any updates occur.
                (self.draw)(1.0);

                // Reset variables that are used for tracking time so that we
                // don't simulate time passed while the application was paused.
                self.last_frame_time_ms = timestamp;
                self.last_fps_update = timestamp;
                self.frames_since_last_fps_update = 0;

                // The application isn't considered "running" until the
                // application starts drawing.
                self.running = true;
            } else {
                // Throttle the frame rate (if minFrameDelay is set to a non-zero value by
                // `MainLoop.setMaxAllowedFPS()`).
                if timestamp < (self.last_frame_time_ms + self.opts.min_frame_delay) {
                    return;
                }

                // frameDelta is the cumulative amount of in-app time that hasn't been
                // simulated yet. Add the time since the last frame. We need to track total
                // not-yet-simulated time (as opposed to just the time elapsed since the
                // last frame) because not all actually elapsed time is guaranteed to be
                // simulated each frame. See the comments below for details.
                self.frame_delta += timestamp - self.last_frame_time_ms;
                self.last_frame_time_ms = timestamp;

                // Run any updates that are not dependent on time in the simulation. See
                // `MainLoop.setBegin()` for additional details on how to use this.
                (self.begin)(timestamp, self.frame_delta);

                // Update the estimate of the frame rate, `fps`. Approximately every
                // second, the number of frames that occurred in that second are included
                // in an exponential moving average of all frames per second. This means
                // that more recent seconds affect the estimated frame rate more than older
                // seconds.
                if timestamp > (self.last_fps_update + self.opts.fps_update_interval) {
                    // Compute the new exponential moving average.
                    self.fps =
                            // Divide the number of frames since the last FPS update by the
                            // amount of time that has passed to get the mean frames per second
                            // over that period. This is necessary because slightly more than a
                            // second has likely passed since the last update.
                            self.opts.fps_alpha * (self.frames_since_last_fps_update as f64) * 1000.0 / (timestamp - self.last_fps_update) + (1.0 - self.opts.fps_alpha) * self.fps;

                    // Reset the frame counter and last-updated timestamp since their
                    // latest values have now been incorporated into the FPS estimate.
                    self.last_fps_update = timestamp;
                    self.frames_since_last_fps_update = 0;
                }
                // Count the current frame in the next frames-per-second update. This
                // happens after the previous section because the previous section
                // calculates the frames that occur up until `timestamp`, and `timestamp`
                // refers to a time just before the current frame was delivered.
                self.frames_since_last_fps_update += 1;

                /*
                 * A naive way to move an object along its X-axis might be to write a main
                 * loop containing the statement `obj.x += 10;` which would move the object
                 * 10 units per frame. This approach suffers from the issue that it is
                 * dependent on the frame rate. In other words, if your application is
                 * running slowly (that is, fewer frames per second), your object will also
                 * appear to move slowly, whereas if your application is running quickly
                 * (that is, more frames per second), your object will appear to move
                 * quickly. This is undesirable, especially in multiplayer/multi-user
                 * applications.
                 *
                 * One solution is to multiply the speed by the amount of time that has
                 * passed between rendering frames. For example, if you want your object to
                 * move 600 units per second, you might write `obj.x += 600 * delta`, where
                 * `delta` is the time passed since the last frame. (For convenience, let's
                 * move this statement to an update() function that takes `delta` as a
                 * parameter.) This way, your object will move a constant distance over
                 * time. However, at low frame rates and high speeds, your object will move
                 * large distances every frame, which can cause it to do strange things
                 * such as move through walls. Additionally, we would like our program to
                 * be deterministic. That is, every time we run the application with the
                 * same input, we would like exactly the same output. If the time between
                 * frames (the `delta`) varies, our output will diverge the longer the
                 * program runs due to accumulated rounding errors, even at normal frame
                 * rates.
                 *
                 * A better solution is to separate the amount of time simulated in each
                 * update() from the amount of time between frames. Our update() function
                 * doesn't need to change; we just need to change the delta we pass to it
                 * so that each update() simulates a fixed amount of time (that is, `delta`
                 * should have the same value each time update() is called). The update()
                 * function can be run multiple times per frame if needed to simulate the
                 * total amount of time passed since the last frame. (If the time that has
                 * passed since the last frame is less than the fixed simulation time, we
                 * just won't run an update() until the the next frame. If there is
                 * unsimulated time left over that is less than our timestep, we'll just
                 * leave it to be simulated during the next frame.) This approach avoids
                 * inconsistent rounding errors and ensures that there are no giant leaps
                 * through walls between frames.
                 *
                 * That is what is done below. It introduces a new problem, but it is a
                 * manageable one: if the amount of time spent simulating is consistently
                 * longer than the amount of time between frames, the application could
                 * freeze and crash in a spiral of death. This won't happen as long as the
                 * fixed simulation time is set to a value that is high enough that
                 * update() calls usually take less time than the amount of time they're
                 * simulating. If it does start to happen anyway, see `MainLoop.setEnd()`
                 * for a discussion of ways to stop it.
                 *
                 * Additionally, see `MainLoop.setUpdate()` for a discussion of performance
                 * considerations.
                 *
                 * Further reading for those interested:
                 *
                 * - http://gameprogrammingpatterns.com/game-loop.html
                 * - http://gafferongames.com/game-physics/fix-your-timestep/
                 * - https://gamealchemist.wordpress.com/2013/03/16/thoughts-on-the-javascript-game-loop/
                 * - https://developer.mozilla.org/en-US/docs/Games/Anatomy
                 */
                self.num_update_steps = 0;

                while self.frame_delta >= self.opts.simulation_timestep {
                    (self.update)(self.opts.simulation_timestep);
                    self.frame_delta -= self.opts.simulation_timestep;

                    /*
                     * Sanity check: bail if we run the loop too many times.
                     *
                     * One way this could happen is if update() takes longer to run than
                     * the time it simulates, thereby causing a spiral of death. For ways
                     * to avoid this, see `MainLoop.setEnd()`. Another way this could
                     * happen is if the browser throttles serving frames, which typically
                     * occurs when the tab is in the background or the device battery is
                     * low. An event outside of the main loop such as audio processing or
                     * synchronous resource reads could also cause the application to hang
                     * temporarily and accumulate not-yet-simulated time as a result.
                     *
                     * 240 is chosen because, for any sane value of simulationTimestep, 240
                     * updates will simulate at least one second, and it will simulate four
                     * seconds with the default value of simulationTimestep. (Safari
                     * notifies users that the script is taking too long to run if it takes
                     * more than five seconds.)
                     *
                     * If there are more updates to run in a frame than this, the
                     * application will appear to slow down to the user until it catches
                     * back up. In networked applications this will usually cause the user
                     * to get out of sync with their peers, but if the updates are taking
                     * this long already, they're probably already out of sync.
                     */
                    self.num_update_steps += 1;
                    if self.num_update_steps >= 240 {
                        self.end_panic = true;
                        break;
                    }

                    /*
                     * Render the screen. We do this regardless of whether update() has run
                     * during this frame because it is possible to interpolate between updates
                     * to make the frame rate appear faster than updates are actually
                     * happening. See `MainLoop.setDraw()` for an explanation of how to do
                     * that.
                     *
                     * We draw after updating because we want the screen to reflect a state of
                     * the application that is as up-to-date as possible. (`MainLoop.start()`
                     * draws the very first frame in the application's initial state, before
                     * any updates have occurred.) Some sources speculate that rendering
                     * earlier in the requestAnimationFrame callback can get the screen painted
                     * faster; this is mostly not true, and even when it is, it's usually just
                     * a trade-off between rendering the current frame sooner and rendering the
                     * next frame later.
                     *
                     * See `MainLoop.setDraw()` for details about draw() itself.
                     */
                    (self.draw)(self.frame_delta / self.opts.simulation_timestep);

                    // Run any updates that are not dependent on time in the simulation. See
                    // `MainLoop.setEnd()` for additional details on how to use this.
                    (self.end)(self.fps, self.end_panic);

                    self.end_panic = false;
                }
            }
    }
}
