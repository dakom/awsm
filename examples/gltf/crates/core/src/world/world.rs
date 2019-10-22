use shipyard::*;
use crate::components::*;
use shared::state::*;

pub fn init_world(window_width: u32, window_height: u32) -> World {
    let world = World::default();

    //These are added immediately 
    world.register::<WindowSize>();
    world.register::<InitState>();

    world.run::<(EntitiesMut, &mut InitState), _>(|(mut entities, mut init_state)| {
        entities.add_entity(
            (&mut init_state), 
            (
                InitState::new()
            )
        );
    });


    world.run::<(EntitiesMut, &mut WindowSize), _>(|(mut entities, mut window_size)| {
        entities.add_entity(
            (&mut window_size), 
            (
                WindowSize {width: window_width, height: window_height} 
            )
        );
    });

    world
}