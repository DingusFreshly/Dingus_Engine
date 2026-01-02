mod instance;
mod components;
mod types;
mod macros;
mod world;

use components::components::*;
use world::World;
pub type Entity = u32;

include_components!(
    shape_component : Shape,
    flat_position_component: FlatPosition,

);

fn main() {
    let mut world = World::new();
    let id = world.spawn();
    //component adding on creation
    add_entity_with_components!(
        world,
        FlatPosition {
            position : Vector2 { x: 1.0, y: 1.0 },
        },
    );
    //component adding after creationg
    let id = world.spawn();
    InsertComponent::insert(
        FlatPosition {
            position : Vector2 { x: 1.0, y: 1.0 },
        },
        id,
        &mut world.component_handler,
    );
    println!("{:?}", world.component_handler.flat_position_component);

}