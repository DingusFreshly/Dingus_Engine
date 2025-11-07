use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, KeyEvent, ElementState},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
    keyboard::PhysicalKey,
};

use crate::framework::DingusFramework;
use crate::render_2d::{Renderer, Shape2D};
use crate::instances::instance_descriptors::Instance;
use std::sync::Arc;
pub struct GameClient {
    
    framework: Option<DingusFramework>,
}

impl GameClient {
    pub fn new() -> Self {
        let framework = None;//DingusFramework::new();

        Self {
            framework,
        }
    }

    pub fn run(mut self) -> Result<(), winit::error::EventLoopError> {
        let event_loop = EventLoop::new()?; // must be here
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run_app(&mut self)?; // gives access to ActiveEventLoop inside
        Ok(())
    }
}

impl ApplicationHandler for GameClient {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = WindowAttributes::default().with_title("heheheha");

        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        
        self.framework = Some(DingusFramework::new(window.clone()));
        
        self.framework.as_mut().unwrap().startup();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, wid: WindowId, event: WindowEvent) {
        let framework = self.framework.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                println!("Window closed!");
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent { physical_key: PhysicalKey::Code(code), state, .. },
                ..
            } => {
                framework.handle_input(event_loop, code, state);
            }
            WindowEvent::RedrawRequested => {
                //println!("Redraw Requested!"); 
                framework.update();
                framework.render();
            }
            _ => {}
        }
    }
}
