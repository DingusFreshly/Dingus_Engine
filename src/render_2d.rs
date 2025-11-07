use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

pub const CHUNK_SIZE: usize = 4;

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    background_color: [u8; CHUNK_SIZE], // RGBA
    pixels: Pixels<'static>,

}

pub enum Shape2D {
    Rect(u32, u32),  // width, height
    Square(u32),     // size
    Circle(u32),     // radius
    Line(i64,i64,i64,i64,)//x1,y1,x2, y2
}

impl  Renderer {
    pub fn new(window: Arc<Window>, background_color: [u8; CHUNK_SIZE]) -> Self {

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, Arc::clone(&window));

        let pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();
        Renderer {
            width: size.width,
            height: size.height,
            background_color,
            pixels,
        }
    }
    pub fn present(&mut self) {
        self.pixels.render().expect("Pixels failed to render");
    }
    pub fn from_center(&self, x: i32, y: i32) -> (i32, i32) {
        let offset_x = self.width as i32 / 2;
        let offset_y = self.height as i32 / 2;
        (offset_x + x, offset_y + y)
    }

    pub fn clear(&mut self) {
        let frame = self.pixels.frame_mut();

        for chunk in frame.chunks_exact_mut(CHUNK_SIZE) {
            chunk.copy_from_slice(&self.background_color);
        }
    }

    fn filter_xy(&self, x: i64, y: i64) -> bool {
        x >= 0 && y >= 0 && x < self.width as i64 && y < self.height as i64
    }

    pub fn draw_pixel(&mut self, x: i64, y: i64, color: [u8; CHUNK_SIZE]) {
        if self.filter_xy(x, y) {
            let frame = self.pixels.frame_mut();

            let idx = ((y * self.width as i64 + x) * CHUNK_SIZE as i64) as usize;
            frame[idx..idx + CHUNK_SIZE].copy_from_slice(&color);
        } else {
            // optional debug: println!("Pixel out of bounds: ({},{})", x, y);
        }
    }

    pub fn draw_shape(&mut self, shape: Shape2D, x: i64, y: i64, color: [u8; CHUNK_SIZE]) {
        match shape {
            Shape2D::Square(size) => {
                for dy in 0..size as i64 {
                    for dx in 0..size as i64 {
                        self.draw_pixel(x + dx, y + dy, color);
                    }
                }
            }
            Shape2D::Rect(w, h) => {
                for dy in 0..h as i64 {
                    for dx in 0..w as i64 {
                        self.draw_pixel(x + dx, y + dy, color);
                    }
                }
            }
            Shape2D::Circle(radius) => {
                let r = radius as i64;
                for dy in -r..=r {
                    for dx in -r..=r {
                        if dx * dx + dy * dy <= r * r {
                            self.draw_pixel(x + dx, y + dy, color);
                        }
                    }
                }
            }
            Shape2D::Line(x1, y1, x2, y2) => {
                let mut x0 = x1;
                let mut y0 = y1;
                let dx = (x2 - x1).abs();
                let dy = -(y2 - y1).abs();
                let sx = if x0 < x2 { 1 } else { -1 };
                let sy = if y0 < y2 { 1 } else { -1 };
                let mut err = dx + dy;

                loop {
                    self.draw_pixel(x0, y0, color);
                    if x0 == x2 && y0 == y2 { break; }
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
