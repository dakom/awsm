use awsm::webgl::{Id};
use super::{Material, Skin, AlphaMode};

pub struct Mesh {
    pub primitives: Option<Vec<Primitive>>,
    pub morph_weights: Option<Vec<f64>>,
    pub skin: Option<Skin>,
}

pub struct Primitive {
    shader_config: PrimitiveShaderConfig,
    vao_id: Id,
    draw_mode: DrawMode,
    material: Option<Material>
}


pub enum DrawMode {
    Elements(Id),
    Array(usize)
}

pub struct PrimitiveShaderConfig {
    n_morph_weights: usize,
    n_position_morphs:usize,
    n_normal_morphs: usize,
    n_tangent_morphs: usize,
    n_skin_joints:usize,
    has_normal_attributes: bool,
    has_tangent_attributes: bool,
    has_uv_attributes: bool,
    has_color_attributes: bool,
    has_base_color_map: bool,
    has_normal_map: bool,
    has_emissive_map: bool,
    has_metal_roughness_map: bool,
    has_occlusion_map: bool,
    manual_srgb: bool,
    fast_srgb: bool,
    alpha_mode: AlphaMode,
    unlit: bool,
}
