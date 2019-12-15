/**
 * Customize this for all the bridge event types
 * If there are any complex objects, create structs on the Rust side too!
 */

//The order of these must match the Rust BridgeEventIndex!
export enum BridgeEvent {
    WindowSize,
    LoadGltf,
    GltfLoaded,
    CameraSettings,
    Clear
}

export type ValidBridgeEvents = 
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
    zfar: number
}

export interface PerspectiveCameraSettings {
    style: CameraStyle,
    aspectRatio: number;
    yfov: number;
    znear: number,
    zfar: number
}
export enum CameraStyle {
    ORTHOGRAPHIC,
    PERSPECTIVE
}
