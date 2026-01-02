mod instance;

pub type Entity = u32;
#[derive(Debug)]
struct Vector2 {
    x : f32,
    y : f32,
}
#[derive(Debug)]
enum ShapeType {
    Plane,
    Circle,
    Polygon,
}
#[derive(Debug)]
struct FlatPosition{
    position: Vector2,
}
#[derive(Debug)]
struct Shape {
    shape: ShapeType
}
struct World {
    flat_position_components: Vec<Option<FlatPosition>>,
    shape_components: Vec<Option<Shape>>,
}
impl World {
    fn new() -> Self {

        World {
            flat_position_components: Vec::new(),
            shape_components: Vec::new(),
        }
    }
    fn add_entity(&mut self, flat_position: Option<FlatPosition>,shape: Option<Shape> ) {
        self.flat_position_components.push(flat_position);
        self.shape_components.push(shape);
    }
}
fn main() {
    let mut world = World::new();
    world.add_entity(
        Some(FlatPosition{position: Vector2 {x:1.0,y:3.0}}),
        Some(Shape{shape: ShapeType::Circle})
    );

    let zipped = world.flat_position_components.iter().zip( world.shape_components.iter());
    let unzipped = zipped.filter_map(|(pos, shape) : (&Option<FlatPosition>,&Option<Shape>)| {
        Some((pos.as_ref()?, shape.as_ref()?))
    });

    for (pos, shape) in unzipped {
        println!("{:?} -> {:?}", pos, shape);
    }
}
trait ComponentType {

}

struct ComponentHandler {
    shape_components: Vec<Shape>,
    flat_position_components: Vec<FlatPosition>,
}
impl ComponentHandler {
    fn new() -> Self {

    }
}