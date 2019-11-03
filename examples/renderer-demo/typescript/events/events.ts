import {set_state} from "@state/state";
import {BridgeEvent, ValidBridgeEvents} from "./bridge-events";
export * from "./bridge-events";


//could be called from anywhere - sends to the wasm/core
export const send_bridge_event = (event:ValidBridgeEvents) => {
    if(typeof event === "number") {
        send_bridge_event_to_core_unchecked(event);
    } else {
        send_bridge_event_to_core_unchecked(event[0], event[1]);
    }
}

//from wasm/core
export const send_bridge_event_from_core_to_ts_unchecked:EventSender = (evt_type:BridgeEvent, evt_data?:any) => {
    switch(evt_type) {
        case BridgeEvent.GltfLoaded: set_state("loaded"); break;
    }
}

//Needed for glue
export const init_core_sender = (fn:EventSender) => {
    send_bridge_event_to_core_unchecked = fn;
}
let send_bridge_event_to_core_unchecked:EventSender;

//just to help the type checker 
type EventSender = (evt_type:BridgeEvent, evt_data?:any) => void;