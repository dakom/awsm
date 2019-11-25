import {samples_path} from "@utils/path";
import dat from "dat.gui";
import {debug_settings} from "@config/config";
import {BridgeEvent, send_bridge_event, CameraStyle} from "@events/events"
import { set_state } from "@state/state";

/*
const model_data:Array<ModelData> = 
    samples_path("model-index.json")
    |> await fetch
    |> await json
    */
export const init_menu = async () => {
    const gui = new dat.GUI();
    const model_data:Array<ModelData> = await (await fetch(samples_path("model-index.json"))).json();

    init_model_menu (model_data);
    init_camera_menu ();

    //helper funcs
    interface ModelData {
        name: string;
        variants: Array<string>
    }
    function init_model_menu(xs:Array<ModelData>) {
        const model_names = xs.map(({name}) => name);
        const variant_names = xs.map(({variants}) => Object.keys(variants));
        const variant_values = xs.map(({variants}) => Object.values(variants));
        let variant_name = variant_names[debug_settings.model_idx][debug_settings.variant_idx];
        let model_name = model_names[debug_settings.model_idx];

        const reload = () => {
            const model_idx = model_names.indexOf(model_name);
            const variant_idx = variant_names[model_idx].indexOf(variant_name);
            const variant_value = variant_values[model_idx][variant_idx];

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
    }

    function init_camera_menu() {
        let cameraFolder;
        const reset_camera = (style:"orthographic" | "perspective") => {
            if(cameraFolder) {
                gui.removeFolder(cameraFolder);
            }

            const opts:any = {style};

            cameraFolder = gui.addFolder("camera");
            cameraFolder.open(); 

            const camera_style = cameraFolder.add(opts, "style", ["orthographic", "perspective"]);
            camera_style.onChange(style => {
                reset_camera(style);
            });

            const add_camera_menu_option = (label:string) => (value:any) => {
                opts[label] = value;
                cameraFolder
                    .add(opts, label)
                    .onChange(value => { 
                        send_camera_settings();
                    });

            }
            const setup_orthographic = () => { 
                add_camera_menu_option("xmag") (1.0);
                add_camera_menu_option("ymag") (1.0);
                add_camera_menu_option("znear") (0.01);
                add_camera_menu_option("zfar") (1.0);
            }

            const setup_perspective = () => { 
            }

            if (style === "orthographic") {
                setup_orthographic();
            } else {
                setup_perspective();
            }
            send_camera_settings();

            function send_camera_settings() {
                if(style === "orthographic") {
                    send_bridge_event([BridgeEvent.CameraSettings, {
                        style: CameraStyle.ORTHOGRAPHIC,
                        xmag: opts.xmag,
                        ymag: opts.ymag,
                        znear: opts.znear,
                        zfar: opts.zfar,
                    }]) 
                }
            }
        }

        reset_camera("orthographic");
    }
}
