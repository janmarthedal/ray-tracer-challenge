use crate::color::{Color, WHITE};

#[derive(Clone, Copy)]
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
    pub fn set_color(&self, color: Color) -> Self {
        Self { color, ..*self }
    }
    pub fn set_ambient(&self, ambient: f64) -> Self {
        Self { ambient, ..*self }
    }
    pub fn set_diffuse(&self, diffuse: f64) -> Self {
        Self { diffuse, ..*self }
    }
    pub fn set_specular(&self, specular: f64) -> Self {
        Self { specular, ..*self }
    }
    pub fn set_shininess(&self, shininess: f64) -> Self {
        Self { shininess, ..*self }
    }
}
