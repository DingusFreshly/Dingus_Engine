use crate::types::*;
use crate::render_2d::Renderer;
use std::any::Any;
use bitflags::bitflags;
use std::hash::{Hash, Hasher};
use crate::instances::instance_manager::InstanceKey;

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct InstanceFlags: u32 {
        const RENDERABLE = 0b0000_0001;
        const PHYSICS = 0b0000_0010;
    }
}
pub trait Instance {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    //returns the name of the instance
    fn class_name(&self) -> &str {
        "Unnamed Instance"
    }
    //converts to dyn Renderable if possible, maybe this is useless now that i have as_any but well see
    fn as_renderable(&self) -> Option<&dyn Renderable> {
        None
    }
}

pub struct InstanceDescriptor {
    pub name: String,
}
pub trait Physics: Instance {
    fn get_properties(&self);
}

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer);
}
//this is used to store hierarchy data of instances
//i just realised this is not needed for now what the hell
pub struct InstanceNode {
    pub name: String,
    
    pub key: InstanceKey,
    
    pub parent: Option<InstanceKey>,
    pub children: Vec<InstanceKey>,
}

impl InstanceNode {
    pub fn find_first_child() {
        todo!()
    }
}