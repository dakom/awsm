/**
 * We don't need anything other than run() from wasm
 * So let's remove the complication and potential slowdown
 * of webpack bundling import and just load directly
 * 
 * https://stackoverflow.com/questions/14521108/dynamically-load-js-inside-js
 *
 * Since we also don't need JS snippets or TS really, we can get a bit more low-level too..
 * the following expects that the module was built like this:
 * wasm-bindgen PATH/foo.wasm --no-typescript --target no-modules --no-modules-global-var VAR --out-dir PATH
 * resulting in PATH/foo.js and path/foo_bg.wasm
 * 
 * 
 */


export const load_wasm = (path:string, global_var:string):Promise<any> => 
    new Promise((resolve, reject) => {
        const script = document.createElement('script');
        script.src = path + ".js";
        script.onload = resolve;
        script.onerror = reject;
        const target = document.body ? document.body : document.head;
        target.appendChild(script);
    })
    .then(res => 
        (window as any)[global_var](path + "_bg.wasm")
    )
    .then(res => 
        (window as any)[global_var].run
    )