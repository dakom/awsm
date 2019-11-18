import {samples_path} from "@utils/path";
import dat from "dat.gui";
import {debug_settings} from "@config/config";
import {BridgeEvent, send_bridge_event, CameraStyle} from "@events/events"
import { set_state } from "@state/state";

const gui = new dat.GUI();

export const init_menu = () => {
    fetch(samples_path("model-index.json"))
        .then(x => x.json())
        .then((xs:Array<any>) => {
            const model_names = xs.map(({name}) => name);
            const variant_names = xs.map(({variants}) => Object.keys(variants));
            const variant_values = xs.map(({variants}) => Object.values(variants));
            let variant_name = variant_names[debug_settings.model_idx][debug_settings.variant_idx];
            let model_name = model_names[debug_settings.model_idx];

            const reload = () => {
                const model_idx = model_names.indexOf(model_name);
                const variant_idx = variant_names[model_idx].indexOf(variant_name);
                const variant_value = variant_values[model_idx][variant_idx] as string;

                const gltf_path = samples_path(`${model_name}/${variant_name}`);

                //console.log("model index is", model_idx, "variant index is", variant_idx);
                set_state("loading");
                send_bridge_event([BridgeEvent.LoadGltf, `${gltf_path}/${variant_value}`]);
            }

            const opts = {
                model: model_name,
                variant: variant_name
            }


            const model_controller = gui.add(opts, "model", model_names);
            let variant_controller;
            const reset_variants = (model_idx:number) => {
                if(variant_controller) {
                    gui.remove(variant_controller);
                }

                const vs = variant_names[model_idx];
                variant_name = vs.indexOf(variant_name) === -1 ? vs[0] : variant_name;
                variant_controller = gui.add(opts, "variant", variant_names[model_idx]);
                variant_controller.setValue(variant_name);
                variant_controller.onChange(n => {
                    variant_name = n;
                    reload();
                });

                reload();
            }

            model_controller.onChange(n => {
                model_name = n;
                const model_idx = model_names.indexOf(model_name);
                reset_variants(model_idx);
            });

            //START WITH INITIAL SETTINGS
            (() => {
                reset_variants(debug_settings.model_idx);
            })();
        })
        .then(() => {
            const opts = {
                style: "orthographic"
            }
            let cameraFolder;
            const reset_camera = (style) => {
                if(cameraFolder) {
                    gui.removeFolder(cameraFolder);
                }
                cameraFolder = gui.addFolder("camera");
                const camera_style = cameraFolder.add(opts, "style", ["orthographic", "perspective"]);
                cameraFolder.open(); 
                camera_style.onChange(style => {
                    reset_camera(style);
                });
                send_bridge_event([BridgeEvent.CameraSettings, {
                    style: style === "orthographic" ? CameraStyle.ORTHOGRAPHIC : CameraStyle.PERSPECTIVE
                }]) 
            }

            reset_camera("orthographic");
        })
    }