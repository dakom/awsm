import * as React from "react";
import {SCENE} from "components/scene/types/Scene-Types";
import "./errors.scss";

interface Props {
    message: string;
    scene:SCENE;
}

export const ErrorView = ({message, scene}:Props) => (
    <div className="error">
        <div className="label">
            {message}
            <br/>
            {`Scene: ${scene}`}
        </div>
    </div>
)
