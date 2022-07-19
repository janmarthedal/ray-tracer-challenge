use crate::{ray::Ray, tuple::new_point};

pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }
    pub fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - new_point(0.0, 0.0, 0.0);

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
}

#[cfg(test)]
mod tests {

    use crate::tuple::new_vector;
    use crate::approx_eq::{ApproxEq, assert_approx_eq};
    use super::*;

    #[test]
    fn test_a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [4.0, 6.0]);
    }

    #[test]
    fn test_a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(new_point(0.0, 1.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [5.0, 5.0]);
    }

    #[test]
    fn test_a_ray_misses_a_sphere() {
        let r = Ray::new(new_point(0.0, 2.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, []);
    }

    #[test]
    fn test_a_ray_originates_inside_a_sphere() {
        let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [-1.0, 1.0]);
    }

    #[test]
    fn test_a_ray_is_behind_a_sphere() {
        let r = Ray::new(new_point(0.0, 0.0, 5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [-6.0, -4.0]);
    }
}
