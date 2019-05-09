import * as React from "react";

interface Props {
}

export const LoaderView = (props:Props) => (
    <div id="loader">
        <div className="loader__text">
            Loading...
        </div>
        <div className="loader__graphic" />
    </div>
)
