use crate::errors::Error;
use crate::renderer::Renderer;
use crate::transform::*;
use crate::components::*;
use shipyard::prelude::*;

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
    /// Adds a node to the scene
    pub fn add_node(&mut self, data:NodeData, parent:Option<Key>, translation: Option<Vector3>, rotation: Option<Quaternion>, scale: Option<Vector3>) -> Result<Key, Error> {
        add_node(&mut self.world.borrow_mut(), data, parent, translation, rotation, scale)
    }

    pub fn set_node_trs(&mut self, node:Key, translation: Option<Vector3>, rotation: Option<Quaternion>, scale: Option<Vector3>) {
        let world = self.world.borrow_mut();

        world.run::<(&mut Translation, &mut Rotation, &mut Scale), _, _>(
            |(mut translations, mut rotations, mut scales)| {
                if let Some((t,r,s)) = (&mut translations, &mut rotations, &mut scales).get(node).iter_mut().next() {
                    if let Some(translation) = translation {
                        t.0.copy_from(&translation);
                    }
                    if let Some(rotation) = rotation {
                        r.0.copy_from(&rotation);
                    }
                    if let Some(scale) = scale {
                        s.0.copy_from(&scale);
                    }
                }
            }
        );
    }
}

//Mostly for internal use - but can also be used to share the ECS outside of renderer
pub fn add_node(world:&mut World, data:NodeData, parent:Option<Key>, translation: Option<Vector3>, rotation: Option<Quaternion>, scale: Option<Vector3>) -> Result<Key, Error> {
    let translation = translation.unwrap_or_default();
    let rotation = rotation.unwrap_or_default();
    let scale = scale.unwrap_or(Vector3::new(1.0, 1.0, 1.0));
    let local_matrix = Matrix4::from_trs(&translation, &rotation, &scale);
    let world_matrix = Matrix4::default();

    if let Some(parent) = parent {
        //TODO - re-arrange all the nodes?
        //probably do *not* need to mess with world matrix, just mark dirty and it'll be updated...
    }

    let node = match data {
        NodeData::Empty => {
            world.run::<(
                EntitiesMut, 
                &mut Node,
                &mut Translation,
                &mut Rotation,
                &mut Scale,
                &mut LocalTransform,
                &mut WorldTransform,
            ), _, _>(|(
                mut entities, 
                mut nodes,
                mut translations,
                mut rotations,
                mut scales,
                mut local_matrices,
                mut world_matrices,
            )| {
                Ok(entities.add_entity(
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
                        LocalTransform(local_matrix),
                        WorldTransform(world_matrix),
                    )
                ))
            })
        }
        NodeData::Camera(projection_matrix) => {
            let camera_view = Matrix4::invert_clone(&local_matrix)?;

            world.run::<(
                EntitiesMut, 
                &mut Node,
                &mut CameraView, 
                &mut CameraProjection,
                &mut Translation,
                &mut Rotation,
                &mut Scale,
                &mut LocalTransform,
                &mut WorldTransform,
            ), _, _>(|(
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
                Ok(entities.add_entity(
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
                        LocalTransform(local_matrix),
                        WorldTransform(world_matrix),
                    )
                ))
            })
        }

        NodeData::Primitive(primitive) => {
            world.run::<(
                EntitiesMut, 
                &mut Node,
                &mut Primitive,
                &mut Translation,
                &mut Rotation,
                &mut Scale,
                &mut LocalTransform,
                &mut WorldTransform,
            ), _, _>(|(
                mut entities, 
                mut nodes,
                mut primitives,
                mut translations,
                mut rotations,
                mut scales,
                mut local_matrices,
                mut world_matrices,
            )| {
                Ok(entities.add_entity(
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
                        LocalTransform(local_matrix),
                        WorldTransform(world_matrix),
                    )
                ))
            })
        }
    };

    node
}