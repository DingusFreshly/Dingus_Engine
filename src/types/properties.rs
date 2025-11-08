use std::collections::HashMap;
use crate::types::{Colour, Vector2};pub enum PropertyValue {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Vector2(Vector2),
    Color(Colour)
}
impl PropertyValue {
    pub fn as_int(&self) -> Option<i64> {
        if let PropertyValue::Int(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_int_mut(&mut self) -> Option<&mut i64> {
        if let PropertyValue::Int(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        if let PropertyValue::Float(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_float_mut(&mut self) -> Option<&mut f64> {
        if let PropertyValue::Float(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_vector2(&self) -> Option<&Vector2> {
        if let PropertyValue::Vector2(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn as_vector2_mut(&mut self) -> Option<&mut Vector2> {
        if let PropertyValue::Vector2(v) = self {
            Some(v)
        } else {
            None
        }
    }
    
}

