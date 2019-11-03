export const buildMode = process.env['NODE_ENV'];
export const buildVersion =  process.env['BUILD_VERSION'];
export const isProduction = buildMode === "production" ? true : false;

export const SamplesUrlBase = !isProduction  ? `http://localhost:4102` : "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0";

interface DebugSettings {
    model_idx: number;
    variant_idx: number;
}

const devDebugSettings:DebugSettings = {
    model_idx: 51, //Triangle
    variant_idx: 0, //Embedded
    //variant_idx: 1, //Embedded
}

const prodDebugSettings:DebugSettings = {
    model_idx: 0,
    variant_idx: 0,
}

export const debug_settings = isProduction ? prodDebugSettings : devDebugSettings;
