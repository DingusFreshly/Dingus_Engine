use std::collections::HashMap;
use crate::types::{Colour, Vector2};pub enum PropertyValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Vector2(Vector2),
    Color(Colour)
}

