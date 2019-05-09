import {createElement as el, Fragment, useState} from "react";
import {Menu} from "components/menu/Menu";
import {Scene} from "components/scene/Scene";
import {debugSettings} from "config/Config";

export const App = () => {
    const [scene, setScene] = useState(debugSettings.scene);

    return el(Fragment, null, [
        el(Scene, {scene, key: "scene"}),
        el(Menu, {
            key: "menu",
            onSelect: setScene,
            menu: scene
        })
    ]);
}
