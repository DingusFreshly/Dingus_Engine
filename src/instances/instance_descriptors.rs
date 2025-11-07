use crate::types::*;
use crate::render_2d::Renderer;
use std::any::Any;
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any { self }
}

pub trait Instance: AsAny {}

pub struct InstanceDescriptor {
    pub name: String,
}
pub trait RigidBody {
    fn get_properties(&self);
}
pub trait Renderable {
    fn render(&self, renderer: &mut Renderer);
}