use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

pub trait Object {
    fn get_id(&self) -> usize;
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
    fn normal_at(&self, p: &Point) -> Vector;
}

pub struct World {
    objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
}
