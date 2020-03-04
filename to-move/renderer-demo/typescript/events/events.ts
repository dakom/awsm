import {set_state} from "@state/state";
/**
 * Customize this for all the bridge event types
 * If there are any complex objects, create structs on the Rust side too!
 */

//The order of these must match the Rust BridgeEvent!
export enum BridgeEvent {
    WindowSize,
    LoadGltf,
    GltfLoaded,
    CameraSettings,
    Clear
}

type ValidEvent = 
    | [BridgeEvent.WindowSize, WindowSize]
    | BridgeEvent.GltfLoaded
    | [BridgeEvent.LoadGltf, string]
    | [BridgeEvent.CameraSettings, OrthographicCameraSettings | PerspectiveCameraSettings]
    | BridgeEvent.Clear

interface WindowSize{
    width: number;
    height: number;
}

export interface OrthographicCameraSettings {
    style: CameraStyle,
    xmag: number,
    ymag: number,
    znear: number,
    zfar: number,
    positionX: number,
    positionY: number,
    positionZ: number,
}

export interface PerspectiveCameraSettings {
    style: CameraStyle,
    aspectRatio: number;
    yfov: number;
    znear: number,
    zfar: number,
    positionX: number,
    positionY: number,
    positionZ: number,
}
export enum CameraStyle {
    ORTHOGRAPHIC,
    PERSPECTIVE
}


//this is loosely defined because the types are converted on the rust side 
type EventSender = (evt_type:number, evt_data:any) => unknown;
let send_event_to_rust:EventSender;


export const send_bridge_event_from_rust_to_ts_unchecked = (evt_type:BridgeEvent, evt_data?:any) => {
    switch(evt_type) {
        case BridgeEvent.GltfLoaded: set_state("loaded"); break;
    }
}
export const send_event = (event:ValidEvent) => {
    send_event_to_rust(event[0], event[1]);
}

export const register_event_sender = (_send_event_to_rust:EventSender) => {
    send_event_to_rust = _send_event_to_rust;
}