import {render} from "lit-html";
import {set_state, State} from "@state/state";
import {init_models_menu} from "./models";
import "./ui.css";

const ui_dom_element= document.getElementById("ui");

let has_inited_menu = false;

export const renderUi = (state:State, interpolation:number) => {
    set_state(state);
    render(ui(), ui_dom_element);


    if(!has_inited_menu) {
        has_inited_menu = true;
        init_models_menu();
    }
}

export const ui = () => {
    return null;
}
