import {createElement as el} from "react";
import {MenuView} from "./view/Menu-View";
import {SCENE} from "components/scene/types/Scene-Types";

interface Props {
    onSelect: (menu:SCENE) => void;
    menu: SCENE;
}

export const Menu = ({onSelect, menu}:Props) => {
    return el(MenuView, {
        selected: menu,
        onSelect: onSelect
    });
}
