use std::collections::HashSet;

use winit::{
    event::{ WindowEvent, ElementState,KeyEvent},
    window::Window,
};
use crate::instances::{instance_manager::InstanceManager, instance_descriptors::Instance};

use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, NamedKey, PhysicalKey};
use std::sync::Arc;
use crate::{startup};
use crate::update;
use crate::render;
use crate::render_2d::Renderer;
use crate::types::properties::PropertyValue;
use std::collections::HashMap;
pub  struct DingusFramework {
    pub input_handler: InputHandler,
    pub renderer: Renderer,
    pub instance_manager: InstanceManager,
    
    pub window: Arc<Window>,
    
    pub game_state: HashMap<String, PropertyValue>,
}
impl DingusFramework {
    pub fn new(window: Arc<Window>) -> DingusFramework{
        let renderer = Renderer::new(window.clone(), [0, 0, 0, 255]);
        let mut game_state = HashMap::new();
        let manager = InstanceManager::new();
        
        DingusFramework {
            instance_manager: manager,
            
            input_handler: InputHandler::new(),
            renderer,
            window, 
            
            game_state,
        }
    }
    
    pub fn startup(&mut self) {
        //self.create_renderer(window);
        // Call startup first
        self.window.request_redraw();
        
        startup(self);

    }
    pub fn update(&mut self) {
        
        update(self);
    }

    pub fn render(&mut self) {
        self.renderer.clear();
        
        render(self);
        self.window.request_redraw();
    }
    pub fn handle_input(&mut self, event_loop : &ActiveEventLoop, key:KeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => {
                //println!("Key pressed: {:?}", key);
                self.input_handler.key_down(key);
            }
            ElementState::Released => {
                //println!("Key released: {:?}", key);
                self.input_handler.key_up(key);
            }
        }

    }


}

pub struct InputHandler {
    pub keys_down: HashSet<KeyCode>,

}
impl InputHandler {
    pub fn new() -> Self {
        Self { keys_down: HashSet::new() }
    }

    pub fn key_down(&mut self, key: KeyCode) {
        self.keys_down.insert(key);
    }
    pub fn key_up(&mut self, key: KeyCode) {
        self.keys_down.remove(&key);
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }
}