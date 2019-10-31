import { render_ui } from "@ui/ui";

type State = "loading" | "loaded";
let _state:State = "loading";

export const set_state = (__state:State) => {
    _state = __state;
    console.log(_state);
    render_ui();
};
export const get_state = ():State => _state;