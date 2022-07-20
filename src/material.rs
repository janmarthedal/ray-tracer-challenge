use crate::color::{Color, WHITE};

pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

pub const DEFAULT_MATERIAL: Material = Material {
    color: WHITE,
    ambient: 0.1,
    diffuse: 0.9,
    specular: 0.9,
    shininess: 200.0,
};

impl Material {
    pub fn new() -> Self {
        Self {
            ..DEFAULT_MATERIAL
        }
    }
}
