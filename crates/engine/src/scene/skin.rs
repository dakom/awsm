pub struct Skin {
    pub gltf_index: Option<usize>,
    pub skeleton_root_index: Option<usize>,
    pub joints: Vec<SkinJoint>,
    pub skin_matrices: Option<Vec<u32>>,
}

pub struct SkinJoint {
    pub gltf_node_index: usize,
    pub inverse_bind_matrix: [f32;16]
}
