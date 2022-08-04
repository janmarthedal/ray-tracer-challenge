use crate::approx_eq::EPSILON;
use crate::point::Point;
use crate::ray::Ray;
use crate::shape::LocalShape;
use crate::vector::Vector;

pub struct Cylinder {}

impl Cylinder {
    pub fn new() -> Self {
        Self {}
    }
}

impl LocalShape for Cylinder {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;

        if a < EPSILON {
            return vec![];
        }

        let b = 2.0 * (ray.origin.x * ray.direction.x + ray.origin.z * ray.direction.z);
        let c = ray.origin.x * ray.origin.x + ray.origin.z * ray.origin.z - 1.0;
        let disc = b * b - 4.0 * a * c;

        if disc < 0.0 {
            return vec![];
        }

        vec![
            (-b - disc.sqrt()) / (2.0 * a),
            (-b + disc.sqrt()) / (2.0 * a),
        ]
    }
    fn local_normal_at(&self, point: &Point) -> Vector {
        Vector::new(point.x, 0.0, point.z)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};

    #[test]
    fn test_a_ray_misses_a_cylinder() {
        let cyl = Cylinder::new();
        assert!(cyl
            .local_intersect(&Ray::new(
                Point::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0)
            ))
            .is_empty());
        assert!(cyl
            .local_intersect(&Ray::new(
                Point::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0)
            ))
            .is_empty());
        assert!(cyl
            .local_intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(1.0, 1.0, 1.0).normalize()
            ))
            .is_empty());
    }

    #[test]
    fn test_a_ray_strikes_a_cylinder() {
        let cyl = Cylinder::new();
        assert_approx_eq!(
            cyl.local_intersect(&Ray::new(
                Point::new(1.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0)
            )),
            [5.0, 5.0]
        );
        assert_approx_eq!(
            cyl.local_intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0)
            )),
            [4.0, 6.0]
        );
        assert_approx_eq!(
            cyl.local_intersect(&Ray::new(
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.1, 1.0, 1.0).normalize()
            )),
            [6.80798, 7.08872]
        );
    }

    #[test]
    fn test_normal_vector_on_a_cylinder() {
        let cyl = Cylinder::new();
        assert_approx_eq!(
            cyl.local_normal_at(&Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        );
        assert_approx_eq!(
            cyl.local_normal_at(&Point::new(0.0, 5.0, -1.0)),
            Vector::new(0.0, 0.0, -1.0)
        );
        assert_approx_eq!(
            cyl.local_normal_at(&Point::new(0.0, -2.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );
        assert_approx_eq!(
            cyl.local_normal_at(&Point::new(-1.0, 1.0, 0.0)),
            Vector::new(-1.0, 0.0, 0.0)
        );
    }
}
