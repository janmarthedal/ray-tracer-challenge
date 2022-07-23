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

#[cfg(test)]
mod tests {

    use crate::approx_eq::{ApproxEq, assert_approx_eq};
    use crate::local_shape::LocalShape;
    use crate::vector::Vector;
    use super::*;

    #[test]
    fn test_the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        assert_approx_eq!(p.local_normal_at(&Point::new(0.0, 0.0, 0.0)), Vector::new(0.0, 1.0, 0.0));
        assert_approx_eq!(p.local_normal_at(&Point::new(10.0, 0.0, -10.0)), Vector::new(0.0, 1.0, 0.0));
        assert_approx_eq!(p.local_normal_at(&Point::new(-5.0, 0.0, 150.0)), Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_a_ray_intersecting_a_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert_approx_eq!(xs, [1.0]);
    }

    #[test]
    fn test_a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert_approx_eq!(xs, [1.0]);
    }
}