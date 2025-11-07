use rand::prelude::*;

pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub enum BrickColour {
    Red,
    Blue,
    Green,
    Yellow,
    Black,
    White,
    Orange,
    Purple,
    Pink,
    Gray,
    Random,
}

impl Colour {
    /// Create a colour from u8 RGB (alpha defaults to 255)
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a colour from u8 RGBA
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
    pub fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Create a Colour from a BrickColour
    pub fn from_brick_colour(brick_colour: BrickColour) -> Self {
        match brick_colour {
            BrickColour::Red => Self::from_rgb(255, 0, 0),
            BrickColour::Blue => Self::from_rgb(0, 0, 255),
            BrickColour::Green => Self::from_rgb(0, 255, 0),
            BrickColour::Yellow => Self::from_rgb(255, 255, 0),
            BrickColour::Black => Self::from_rgb(0, 0, 0),
            BrickColour::White => Self::from_rgb(255, 255, 255),
            BrickColour::Orange => Self::from_rgb(255, 128, 0),
            BrickColour::Purple => Self::from_rgb(128, 0, 128),
            BrickColour::Pink => Self::from_rgb(255, 192, 204),
            BrickColour::Gray => Self::from_rgb(128, 128, 128),
            BrickColour::Random => {
                let mut rng = rand::rng();
                let (r, g, b) = (rng.gen_range(0.0..=255.0), rng.gen_range(0.0..=255.0), rng.gen_range(0.0..=255.0));
                Self::from_rgb(r as u8, g as u8, b as u8)
            },
        }
    }

}