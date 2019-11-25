use crate::renderer::Renderer;
use crate::transform::*;
use crate::components::*;
use shipyard::*;

pub struct Node {
    //just an idea so far to try and keep flat list... might not pan out
    pub n_children: usize,
}
impl Node {
    pub fn new() -> Self {
        Self{
            n_children: 0
        }
    }
}

pub enum NodeData {
    Empty,
    Camera(Matrix4), //Projection matrix. View Matrix is calculated from trs
    Primitive(Primitive)
}

impl Renderer {
    pub fn add_node(&mut self, data:NodeData, parent:Option<Key>, translation: Option<Vector3>, rotation: Option<Quaternion>, scale: Option<Vector3>) -> Key {
        add_node(&mut self.world.borrow_mut(), data, parent, translation, rotation, scale)
    }
}

//Mostly for internal use - but can also be used to share the ECS outside of renderer
pub fn add_node(world:&mut World, data:NodeData, parent:Option<Key>, translation: Option<Vector3>, rotation: Option<Quaternion>, scale: Option<Vector3>) -> Key {
    let translation = translation.unwrap_or_default();
    let rotation = rotation.unwrap_or_default();
    let scale = scale.unwrap_or(Vector3::new(1.0, 1.0, 1.0));
    let local_matrix = Matrix4::from_trs(&translation, &rotation, &scale);
    let world_matrix = Matrix4::default();
    let mut node:Option<Key> = None;

    if let Some(parent) = parent {
        //TODO - re-arrange all the nodes?
        //probably do *not* need to mess with world matrix, just mark dirty and it'll be updated...
    }

    match data {
        NodeData::Empty => {
            world.run::<(
                EntitiesMut, 
                &mut Node,
                &mut Translation,
                &mut Rotation,
                &mut Scale,
                &mut LocalMatrix,
                &mut WorldMatrix,
            ), _>(|(
                mut entities, 
                mut nodes,
                mut translations,
                mut rotations,
                mut scales,
                mut local_matrices,
                mut world_matrices,
            )| {
                node = Some(entities.add_entity(
                    (
                        &mut nodes,
                        &mut translations,
                        &mut rotations,
                        &mut scales,
                        &mut local_matrices,
                        &mut world_matrices,
                    ), 
                    (
                        Node::new(),
                        Translation(translation),
                        Rotation(rotation),
                        Scale(scale),
                        LocalMatrix(local_matrix),
                        WorldMatrix(world_matrix),
                    )
                ));
            });
        }
        NodeData::Camera(projection_matrix) => {
            let camera_view = local_matrix.clone(); //TODO - should be inverse of this, I think?
            world.run::<(
                EntitiesMut, 
                &mut Node,
                &mut CameraView, 
                &mut CameraProjection,
                &mut Translation,
                &mut Rotation,
                &mut Scale,
                &mut LocalMatrix,
                &mut WorldMatrix,
            ), _>(|(
                mut entities, 
                mut nodes,
                mut camera_views, 
                mut camera_projections,
                mut translations,
                mut rotations,
                mut scales,
                mut local_matrices,
                mut world_matrices,
            )| {
                node = Some(entities.add_entity(
                    (
                        &mut nodes,
                        &mut camera_views, 
                        &mut camera_projections,
                        &mut translations,
                        &mut rotations,
                        &mut scales,
                        &mut local_matrices,
                        &mut world_matrices,
                    ), 
                    (
                        Node::new(),
                        CameraView(camera_view),
                        CameraProjection(projection_matrix),
                        Translation(translation),
                        Rotation(rotation),
                        Scale(scale),
                        LocalMatrix(local_matrix),
                        WorldMatrix(world_matrix),
                    )
                ));
            });
        }

        NodeData::Primitive(primitive) => {
            world.run::<(
                EntitiesMut, 
                &mut Node,
                &mut Primitive,
                &mut Translation,
                &mut Rotation,
                &mut Scale,
                &mut LocalMatrix,
                &mut WorldMatrix,
            ), _>(|(
                mut entities, 
                mut nodes,
                mut primitives,
                mut translations,
                mut rotations,
                mut scales,
                mut local_matrices,
                mut world_matrices,
            )| {
                node = Some(entities.add_entity(
                    (
                        &mut nodes,
                        &mut primitives, 
                        &mut translations,
                        &mut rotations,
                        &mut scales,
                        &mut local_matrices,
                        &mut world_matrices,
                    ), 
                    (
                        Node::new(),
                        primitive,
                        Translation(translation),
                        Rotation(rotation),
                        Scale(scale),
                        LocalMatrix(local_matrix),
                        WorldMatrix(world_matrix),
                    )
                ));
            });
        }
    }

    node.unwrap()
}