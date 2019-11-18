/**
 * Customize this for all the bridge event types
 * If there are any complex objects, create structs on the Rust side too!
 */

//The order of these must match the Rust BridgeEventIndex!
export enum BridgeEvent {
    WindowSize,
    LoadGltf,
    GltfLoaded,
    CameraSettings
}

export type ValidBridgeEvents = 
    | [BridgeEvent.WindowSize, WindowSize]
    | BridgeEvent.GltfLoaded
    | [BridgeEvent.LoadGltf, string]
    | [BridgeEvent.CameraSettings, CameraSettings ]

interface WindowSize{
    width: number;
    height: number;
}

export interface CameraSettings {
    style: CameraStyle
}

export enum CameraStyle {
    ORTHOGRAPHIC,
    PERSPECTIVE
}
