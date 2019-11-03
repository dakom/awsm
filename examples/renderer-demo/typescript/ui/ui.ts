import {render, html} from "lit-html";
import "./ui.css";
import { get_state, set_state } from "@state/state";

const ui_dom_element= document.getElementById("ui");
export const init_ui = () => {
    set_state("loading");
}

export const render_ui = () => {
    const ui = get_state() === "loading" ? loading() : null;
    render(
        html`
            <div class="ui">
                ${ui}
            </div>
        `, ui_dom_element
    );
}

const loading = () => html`
    <div class="loading">
        <div class="text">Loading...</div>
    </div>`;