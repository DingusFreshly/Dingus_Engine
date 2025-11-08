mod framework;
mod client;
mod render_2d;
mod types;
mod instances;

use framework::*;
use client::GameClient;
use std::io::{self, Write};
use crate::instances::flat_body::Shape2D;
use winit::keyboard::KeyCode;
use types::properties::PropertyValue;
use crate::instances::flat_body::FlatBody;
use crate::types::*;

//runs once at the start of the game
//use this function to define any properties when the play button is pressed
pub fn startup(framework: &mut DingusFramework) {
    println!("Game starting up!");
    io::stdout().flush().unwrap();
    //game state is a hash map of property value enums,
    let game_state = &mut framework.game_state;
    let instance_manager = &mut framework.instance_manager;

    let flat_body = FlatBody{
        shape: Shape2D::Circle(16),
        position: Vector2 { x: 100.0, y: 100.0 },
        color: [255, 0, 0, 255],
    };

    instance_manager.add(flat_body, Some("Circle".to_string()), None);

    //custom vector class
    let position = Vector2 { x: 0.0, y: 0.0 };
    //insert it it into the properties table```````
    let velocity = Vector2 { x: 0.0, y: 0.0 };
    game_state.insert("velocity".to_string(), PropertyValue::Vector2(velocity));
}

pub fn update(framework : &mut DingusFramework) {
    let input = &mut framework.input_handler;

    for i in input.keys_down.iter() {
        println!("Key down: {:?}", i);
    }
    let instance_manager = &mut framework.instance_manager;
    let game_state = &mut framework.game_state;

    let [velocity] = game_state.get_disjoint_mut(["velocity"]);
    let [vel] = [velocity.unwrap().as_vector2_mut().unwrap()];

    if input.is_key_down(KeyCode::KeyW) {
        vel.y -= 1.0;
    }
    if input.is_key_down(KeyCode::KeyS) {
        vel.y += 1.0;
    }
    if input.is_key_down(KeyCode::KeyA) {
        vel.x -= 1.0;
    }
    if input.is_key_down(KeyCode::KeyD) {
        vel.x += 1.0;
    }

    let circle_instance = instance_manager.get_instance_by_name("Circle");
    let mut borrowed = circle_instance.borrow_mut();

    if let Some(flat_body) = borrowed.as_any_mut().downcast_mut::<FlatBody>() {
        flat_body.position.x += vel.x;
        flat_body.position.y += vel.y;
    }
    vel.x *= 0.9;
    vel.y *= 0.9;

    //println!("Updating!");
}
/*this is the render step, its not really needed now
because i implemented rendering with the instances,
but this is here for flexibility if you dont want to
go thorugh the instance bloat
 */
pub fn render(framework: &mut DingusFramework) {
    let renderer = &mut framework.renderer;

    let game_state = &framework.game_state;



    renderer.present();
}


fn main() {
    let target_fps : f64 = 20.0;//useless value for now
    let mut client = GameClient::new();
    client.run();

}