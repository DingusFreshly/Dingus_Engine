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

/*
TODO: Implement delta time pls
 */

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
    game_state.insert("position".to_string(), PropertyValue::Vector2(position));

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

    let [position, velocity] = game_state.get_disjoint_mut(["position", "velocity"]);
    let [pos, vel] = [position.unwrap().as_vector2_mut().unwrap(), velocity.unwrap().as_vector2_mut().unwrap()];
    
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
    pos.x += vel.x;
    pos.y += vel.y;

    let circle_instance = instance_manager.get_instance_by_name("Circle");
    let mut borrowed = circle_instance.borrow_mut();
    
    if let Some(flat_body) = borrowed.as_any_mut().downcast_mut::<FlatBody>() {
        flat_body.position.x += vel.x;
        flat_body.position.y += vel.y;
        //flat_body.color = [0, 255, 0, 255]; 
    }
    vel.x *= 0.9;
    vel.y *= 0.9;

    //println!("Updating!");
}

pub fn render(framework: &mut DingusFramework) {
    let renderer = &mut framework.renderer;
    
    renderer.present();
}


fn main() {
    //let target_fps : f64 = 20.0;
    let client = GameClient::new();
    client.run();

}

/*
old psuedo code ahh 

use game::*;
use instance::*;//used for creating elements of the game
use dingus_vector::Vector3;

fn update(dt : f64) {
    println("Delta time: {}", dt);
}

fn main() {
    //creates a new game instance, maybe could have an application create multiple of these
    let game = game::new("");

    let scene = game.create_scene("main");//name, comes with a default hierarchy or types

    let part = instance::new(InstanceType::Part, "part1");//instance type, name
    part.set_position(Vector3(0.0, 10.0, 0.0));//sets the position of the part
    scene.parent_instance(part, );

}
 */
