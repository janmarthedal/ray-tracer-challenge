use crate::ray::Ray;
use crate::tuple::Tuple;

pub trait Object {
    fn get_id(&self) -> usize;
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
    fn normal_at(&self, p: &Tuple) -> Tuple;
}

pub struct World {
    objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
}
