/* tslint:disable */
/* eslint-disable */
/**
* @param {any} canvas 
* @param {number} window_width 
* @param {number} window_height 
* @param {any} send_bridge_event 
* @returns {any} 
*/
export function run(canvas: any, window_width: number, window_height: number, send_bridge_event: any): any;

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path?: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        