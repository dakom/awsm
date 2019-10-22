/**
 * workers-as-modules is not supported in most browsers so far
 * and the default wasm_bindgen is to use proper es modules
 * so this little shim essentially allows loading the wasm as a worker
 * see the --no-modules flag docs at https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html
 * also note that it's build with --no-modules-global-var wasm_core to avoid global namespace collisions
 */

self.importScripts("./wasm/core/pkg/my_core.js");
const {run} = wasm_core;

(async () => { 
    await wasm_core("./wasm/core/pkg/my_core_bg.wasm");

    let send_event;

    /**
     *  Wasm is ready, now setup communication with the main thread
     *  only 2 types of events are processed:
     *  1. READY (for setup)
     *  2. EVENT (for events sent from the main thread)
     */
    self.onmessage = msg => {
        if(msg.data) {
            if(msg.data.type === "READY") {
				send_event = run(on_draw, msg.data.windowSize.width, msg.data.windowSize.height);
            } else if(send_event && msg.data.type === "EVENT") {
				send_event(msg.data.evt_type, msg.data.evt_data);
            }
        }
    };

    //tell the main thread we're ready
    self.postMessage({
        type: "READY"
    });
})();

function on_draw(data) {
    self.postMessage({ type: "STATE", data });
}