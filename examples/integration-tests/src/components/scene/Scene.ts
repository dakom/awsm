import {createElement as el, Fragment, useState, useRef, useEffect, createRef} from "react";
import {SCENE, sceneIdLookup} from "./types/Scene-Types";
import {LoaderView} from "components/loader/view/Loader-View";
import {ErrorView} from "components/error/view/Error-View";
import {SceneView} from "./view/Scene-View";

type WasmLib = typeof import("../../../target/integration_tests");

interface Props {
    scene: SCENE;
}

export const Scene = ({scene}:Props) => {
    const canvasRef = createRef<HTMLCanvasElement>();
    const [phase, setPhase] = useState(PHASE.LOADING);
    const [error, setError] = useState("");

    useEffect(() => {
        //little ugly that this gets assigned from within the promise...
        //but whatever
        let cleanup;

        loadWasm().then(wasmLib => 
            wasmLib.run(
                canvasRef.current, 
                sceneIdLookup.get(scene), 
            )
        )
        .then(_cleanup => {
            cleanup = _cleanup;
            setPhase(PHASE.READY);
        })
        .catch(err => {
            const errorMessage = err instanceof Error ? err.message
                : err instanceof Event ? "error event (see console)"
                : typeof err === "string" ? err
                : "unknown error";

            console.error(err);
            setPhase(PHASE.ERROR);
            setError(errorMessage);
        })

        return () => {
            console.log("cleaning up...");
            if(cleanup) {
                cleanup();
            }
        }
    }, [scene]);


    const sceneElement = el(SceneView, {canvasRef, key: "scene"});

    return phase === PHASE.LOADING ? el(Fragment, null, [sceneElement, el(LoaderView, {key: "loader"})])
        : phase === PHASE.ERROR ? el(Fragment, null, [sceneElement, el(ErrorView, {key: "error", message: error, scene})])
        : sceneElement;
        

}

enum PHASE {
    LOADING = "loading",
    ERROR = "error",
    READY = "ready"
}

const loadWasm = (() => {
    let _loader:Promise<WasmLib>;
    let _wasmLib:WasmLib;

    const getLib = ():Promise<WasmLib> => new Promise(resolve => {
           _wasmLib !== undefined
               ? resolve(_wasmLib)
               : resolve(getLoader())
    });

    const getLoader = () => {
        if(_loader === undefined) {
            _loader = import("../../../target/integration_tests");
            _loader.then(wasmLib => _wasmLib = wasmLib);
        }

        return _loader;
    }

    return getLib;
})();
