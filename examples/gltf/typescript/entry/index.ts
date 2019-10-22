
import {init_events, send_event, send_event_unchecked, IoEvent} from "@events/events";
import {State} from "@state/state";
import {renderUi} from "@ui/ui";
import {get_window_size} from "@utils/window";
import {get_audio_context} from "@utils/audio";
import {load_wasm} from "@utils/wasm";
import MainLoop from "mainloop.js";
import "./index.css";

//import {set_audio_state, get_audio_state, update_audio} from "audio/audio";


const app_worker = new Worker("core-worker-shim.js");
/**
 * Tell the event sender where we're sending to
 */
init_events(app_worker);

//these really just exists in Rust
//only reason we need it here is because rendering has to be on main thread
//so we need to shuttle it between worker and wasm
let renderWebGl:(state:State, interpolation:number) => void;

//current state
let state:State;

/**
 * Initialize communication with the worker
 * Only two types of incoming events are processed:
 * 1. READY - just to kick things off
 * 2. STATE (for each state type) - when the worker has sent us a new state that needs to be dispatched
 * 
 */
app_worker.onmessage = (msg: MessageEvent) => {
    if (msg.data && msg.data.type) {
        switch (msg.data.type) {
            case "READY": 
            {
                window.onresize = () => {
                    const windowSize = get_window_size();
                    send_event([IoEvent.WindowSize, windowSize]);
                }

                const windowSize = get_window_size();
                app_worker.postMessage({
                    type: "READY",
                    windowSize
                });


                /**
                 * 
                 * Load the renderer WASM into this thread
                 * It'll give us the render function which we call
                 * Every tick, if there's a fresh render_state
                 * 
                 * It's only imported once the worker is ready so it can send events right away
                 */

                load_wasm("wasm/renderer/pkg/my_renderer", "wasm_renderer")
                    .then(run => {
                        const canvas_dom_element = document.getElementById("canvas");
                        const { width, height } = get_window_size();

                        return run(canvas_dom_element, width, height, send_event_unchecked)
                    })
                    .then(_renderWebGl => renderWebGl = _renderWebGl)
                    .then(() => {
                        startMainLoop();
                    });
            } break;

            case "STATE": {
                state = msg.data.data; 
                break;
            }

        }
    }
}

//Main game loop
function startMainLoop() {
    MainLoop
        .setBegin((timestamp, delta) => {
            send_event([IoEvent.LoopBegin, [timestamp, delta]]);
        })
        .setUpdate(delta => {
            send_event([IoEvent.LoopUpdate, delta]);
        })
        .setDraw(interpolation => {
            //send_event([IoEvent.LoopDraw, interpolation]);
            if(state) {
                if(renderWebGl) {
                    renderWebGl(state, interpolation);
                }

                if(renderUi) {
                    renderUi(state, interpolation);
                }
            } else {
                //console.log("MISSED FRAME!");
            }
            state = undefined;
        })
        .setEnd((fps, panic) => {
            send_event([IoEvent.LoopEnd, [fps, panic]]);
        })
        .start();
}


send_event([IoEvent.WindowSize, get_window_size()]);