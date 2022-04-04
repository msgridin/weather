use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub city: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.city, self.country)
    }
}
impl Default for Location {
    fn default() -> Self {
        Location {
            city: String::new(),
            country: "".to_string(),
            lat: 0.0,
            lon: 0.0,
        }
    }
}