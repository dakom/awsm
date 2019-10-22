/**
 * Make sure this matches state in rust shared!
 */
export interface State {
    audio_active: boolean, 
    renderer_active: boolean,
    speed: number,
    init_phase?: InitPhase, 
    window_width: number,
    window_height: number,
    ball_position_x: number,
    ball_position_y: number,
    collision: boolean
}

//These need to match the order on the rust side
export enum InitPhase {
    Waiting,
    Loading,
    Ready
}

let state:State;

export const get_state = ():Readonly<State> => state;
export const set_state = (_state:State) => state = _state;