import { init_core_sender, send_bridge_event_from_core_to_ts_unchecked, send_bridge_event, BridgeEvent } from "@events/events";
import {init_ui} from "@ui/ui";
import {init_models_menu} from "@ui/models";
import {set_state} from "@state/state";
import { load_wasm } from "@utils/wasm";
import { get_window_size } from "@utils/window";
import "./index.css";

init_ui();

//also load the core wasm immediately
load_wasm("wasm/core/pkg/my_core", "wasm_core")
    .then(init_core => {
        const canvas_dom_element = document.getElementById("canvas");
        const { width, height } = get_window_size();
        window.onresize = () => {
            send_bridge_event([BridgeEvent.WindowSize, get_window_size()]);
        }

        //when the core has finished loading, it'll send an event (via send_bridge_event_to_ts which is just send_bridge_event on the rust side)
        //that event will cause a state transition and then we're off to the races
        init_core_sender(init_core(canvas_dom_element, width, height, send_bridge_event_from_core_to_ts_unchecked));

        init_models_menu();
    })