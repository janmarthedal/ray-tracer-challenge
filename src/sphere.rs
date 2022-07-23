use crate::local_shape::LocalShape;
use crate::point::{Point, ORIGIN};
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }
}

impl LocalShape for Sphere {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - &ORIGIN;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let sqrt_disc = discriminant.sqrt();
            let t1 = (-b - sqrt_disc) / (2.0 * a);
            let t2 = (-b + sqrt_disc) / (2.0 * a);
            vec![t1, t2]
        }
    }
    fn local_normal_at(&self, object_point: &Point) -> Vector {
        object_point - &ORIGIN
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};

    #[test]
    fn test_a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_approx_eq!(xs, [4.0, 6.0]);
    }

    #[test]
    fn test_a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_approx_eq!(xs, [5.0, 5.0]);
    }

    #[test]
    fn test_a_ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_approx_eq!(xs, []);
    }

    #[test]
    fn test_a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_approx_eq!(xs, [-1.0, 1.0]);
    }

    #[test]
    fn test_a_ray_is_behind_a_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_approx_eq!(xs, [-6.0, -4.0]);
    }

    #[test]
    fn test_the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(&Point::new(1.0, 0.0, 0.0));
        assert_approx_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(&Point::new(0.0, 1.0, 0.0));
        assert_approx_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(&Point::new(0.0, 0.0, 1.0));
        assert_approx_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let c = 3f64.sqrt() / 3.0;
        let n = s.local_normal_at(&Point::new(c, c, c));
        assert_approx_eq!(n, Vector::new(c, c, c));
    }

}
