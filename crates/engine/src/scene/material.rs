pub struct Material {
  metallic_roughness_values: [f32;2],
  base_color_factor: [f32;4],
  base_color_sampler_index: Option<u32>,
  metallic_roughness_sampler_index: Option<u32>,
  normal: Option<NormalMaterial>,
  occlusion: Option<OcclusionMaterial>,

  emissive_factor: Option<[f32;3]>,
  emissive_sampler_index: Option<u32>,

  alpha_mode: Option<AlphaMode>, 
  alpha_cutoff: Option<f32>,
  double_sided: Option<bool>,
}

pub struct NormalMaterial {
    scale: f64,
    sampler_index: u32,
}

pub struct OcclusionMaterial {
    strength: f64,
    sampler_index: u32,
}

pub enum AlphaMode {
    Opaque,
    Mask,
    Blend
}

pub struct TextureInfo {
    index: Option<u32>,
    coord: Option<f32>,
}

pub struct PbrMetallicRoughnessMaterial {
    base_color_factor: Option<[f32;4]>,
    base_color_texture: Option<TextureInfo>,
    metallic_factor: Option<f32>,
    roughness_factor: Option<f32>,
    metallic_roughness_texture: Option<TextureInfo>
}

pub struct NormalTextureInfo {
    texture_info: TextureInfo,
    scale: Option<f32>
}

pub struct OcclusionTextureInfo {
    texture_info: TextureInfo,
    strength: Option<f32>
}

