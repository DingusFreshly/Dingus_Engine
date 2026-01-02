use crate::{ComponentHandler, Entity};

pub struct World {
    pub component_handler: ComponentHandler,
    pub next_entity: Entity,
}
impl World {
    pub fn new() -> Self {

        World {
            component_handler: ComponentHandler::new(),
            next_entity: 0,
        }
    }
    /*
    spawns an entity with no components
    use the add_entity_to_world! macro to spawn with components or
    
    
     */
    pub fn spawn(&mut self) -> Entity {
        let id = self.next_entity;
        self.next_entity += 1;
        self.component_handler.ensure_entity(id);
        id
    }
}
#[macro_export]
macro_rules! add_entity_with_components {
    ($world:expr, $( $component:expr),* $(,)? ) => {
        let id = $world.spawn();
        $(
            InsertComponent::insert($component, id, &mut $world.component_handler);
        )*
        id
    }
}

#[macro_export]
macro_rules! query_by_components {
    ($world:expr, $($component:expr),* $(,)?) => {
        let mut list = Vec::new();
        let component_handler = &$world.component_handler;
        $(
          for i in 0.. $world.next_entity {
              if component_handler.$component.get(i).is_some() {
                  list.push(i);
              } else if list.get(i){
                  
              }
          }
        )*;
        
    }
}