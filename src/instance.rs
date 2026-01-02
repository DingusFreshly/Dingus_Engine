use crate::Entity;
struct Instance {

    entity_id: Entity,

    parent: Option<Entity>,
    children: Vec<Entity>,
}
