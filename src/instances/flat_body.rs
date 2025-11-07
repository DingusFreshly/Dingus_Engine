use crate::instances::*;
use crate::instances::instance_descriptors::{InstanceDescriptor, Renderable, Instance};
use crate::render_2d::Renderer;
use crate::types::Vector2;

#[derive(Clone, Debug)]
pub enum Shape2D {
    Rect(u32, u32),  // width, height
    Square(u32),     // size
    Circle(u32),     // radius
    Line(Vector2, Vector2), // start, end
}

pub struct FlatBody {
    //pub instance_descriptor: InstanceDescriptor
    pub shape: Shape2D,
    
    pub position: Vector2,
    pub color: [u8; 4],
}
impl FlatBody{
    fn new() -> Self {
        FlatBody {
            //instance_descriptor: InstanceDescriptor { name: String::from("FlatBody") },
            shape: Shape2D::Square(10),
            position: Vector2 { x: 0.0, y: 0.0 },
            color: [255, 255, 255, 255],
        }
    }
}
impl Instance for FlatBody {
    
}

impl Renderable for FlatBody {
    fn render(&self, renderer: &mut Renderer) {
        let (x, y) = (self.position.x as i64, self.position.y as i64);
        let color = self.color;

        match &self.shape {
            Shape2D::Square(size) => {
                for dy in 0..*size as i64 {
                    for dx in 0..*size as i64 {
                        renderer.draw_pixel(x + dx, y + dy, color);
                    }
                }
            }
            Shape2D::Rect(w, h) => {
                for dy in 0..*h as i64 {
                    for dx in 0..*w as i64 {
                        renderer.draw_pixel(x + dx, y + dy, color);
                    }
                }
            }
            Shape2D::Circle(radius) => {
                let r = *radius as i64;
                for dy in -r..=r {
                    for dx in -r..=r {
                        if dx * dx + dy * dy <= r * r {
                            renderer.draw_pixel(x + dx, y + dy, color);
                        }
                    }
                }
            }
            Shape2D::Line(start, end) => {
                // Bresenham's line algorithm
                let (mut x0, mut y0) = (start.x as i64, start.y as i64);
                let (x1, y1) = (end.x as i64, end.y as i64);
                let dx = (x1 - x0).abs();
                let dy = -(y1 - y0).abs();
                let sx = if x0 < x1 { 1 } else { -1 };
                let sy = if y0 < y1 { 1 } else { -1 };
                let mut err = dx + dy;

                loop {
                    renderer.draw_pixel(x + x0, y + y0, color);
                    if x0 == x1 && y0 == y1 {
                        break;
                    }
                    let e2 = 2 * err;
                    if e2 >= dy {
                        err += dy;
                        x0 += sx;
                    }
                    if e2 <= dx {
                        err += dx;
                        y0 += sy;
                    }
                }
            }
        }
    }
}
