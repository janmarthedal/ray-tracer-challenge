use crate::approx_eq::EPSILON;
use crate::local_shape::LocalShape;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Plane {}

impl Plane {
    pub fn new() -> Self {
        Self {}
    }
}

impl LocalShape for Plane {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        if ray.direction.y.abs() < EPSILON {
            return vec![];
        }
        let t = -ray.origin.y / ray.direction.y;
        vec![t]
    }
    fn local_normal_at(&self, _object_point: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}
