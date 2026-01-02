#[derive(Debug, Clone, Default)]
pub struct Vector2 {
    pub x : f32,
    pub y : f32,
}
#[derive(Debug, Clone)]
enum ShapeType {
    Plane,
    Circle,
    Polygon,
}
#[derive(Debug, Clone,Default)]
pub struct FlatPosition{
    pub position: Vector2,
}
#[derive(Debug, Clone)]
pub struct Shape {
    shape: ShapeType
}
pub struct ComponentHandler {
    pub shape_components: Vec<Option<Shape>>,
    pub flat_position_components: Vec<Option<FlatPosition>>,
    pub color_components: Vec<i32>,
    
}

#[macro_export]
macro_rules! include_components {
    (
        $(
            $name:tt : $ty:ty
        ),* $(,)?
    ) => {
        
        pub struct ComponentHandler {
            $(
                pub $name: Vec<Option<$ty>>,
            )*
            pub next_entity: Entity,
        }

        impl ComponentHandler {
            pub fn new() -> Self {
                Self {
                    $(
                        $name: Vec::new(),
                    )*
                    next_entity: 0,
                }
            }
            pub fn ensure_entity(&mut self, id: Entity) {
                let id = id as usize;
                $(
                    if self.$name.len() <= id {
                        self.$name.resize(id + 1, None);
                    }
                )*
            }
        }
        pub trait InsertComponent {
            fn insert(self, id: Entity, handler: &mut ComponentHandler);
        }
        $(
            impl InsertComponent for $ty {
                fn insert(self, id: Entity, handler: &mut ComponentHandler) {
                    handler.$name[id as usize] = Some(self);
                }
            }
        )*
    };
}
