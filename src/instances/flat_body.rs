use std::any::Any;
use std::hash::{Hash, Hasher};

use crate::instances::*;
use crate::instances::instance_descriptors::{InstanceDescriptor, Renderable, Instance, InstanceFlags};
use crate::render_2d::Renderer;
use crate::types::Vector2;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Shape2D {
    Rect(u32, u32),
    Square(u32),
    Circle(u32),
    Line(Vector2, Vector2),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FlatBody {
    pub shape: Shape2D,
    pub position: Vector2,
    pub color: [u8; 4],
}

impl FlatBody {
    pub fn new() -> Self {
        FlatBody {
            shape: Shape2D::Square(10),
            position: Vector2 { x: 0.0, y: 0.0 },
            color: [255, 255, 255, 255],
        }
    }
}

//
// ─── INSTANCE IMPLEMENTATION ───────────────────────────────────────────────
//
impl Instance for FlatBody {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn class_name(&self) -> &str {
        "FlatBody"
    }
    fn as_renderable(&self) -> Option<&dyn Renderable> {
        Some(self)
    }
}

//
// ─── RENDERABLE IMPLEMENTATION ─────────────────────────────────────────────
//says how to draw ts
impl Renderable for FlatBody {
    fn render(&self, renderer: &mut Renderer) {
        println!("RENDERING FLATBODY");
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
