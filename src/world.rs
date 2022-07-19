use crate::ray::Ray;

pub trait Object {
    fn get_id(&self) -> usize;
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
}

pub struct World {
    objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
}
