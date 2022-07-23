use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

pub trait LocalShape {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64>;
    fn local_normal_at(&self, p: &Point) -> Vector;
}
