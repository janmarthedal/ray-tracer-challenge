use crate::matrix::{Matrix, IDENTITY4};
use crate::ray::Ray;
use crate::tuple::new_point;
use crate::world::Object;

pub struct Sphere {
    id: usize,
    transform: Matrix<4>,
}

impl Sphere {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            transform: IDENTITY4,
        }
    }
    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }
}

impl Object for Sphere {
    fn get_id(&self) -> usize {
        self.id
    }
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let ray = ray.transform(&self.transform.inverse().unwrap());
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

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::transform::{translation, scaling};
    use crate::tuple::new_vector;

    #[test]
    fn test_a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [4.0, 6.0]);
    }

    #[test]
    fn test_a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(new_point(0.0, 1.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [5.0, 5.0]);
    }

    #[test]
    fn test_a_ray_misses_a_sphere() {
        let r = Ray::new(new_point(0.0, 2.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, []);
    }

    #[test]
    fn test_a_ray_originates_inside_a_sphere() {
        let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [-1.0, 1.0]);
    }

    #[test]
    fn test_a_ray_is_behind_a_sphere() {
        let r = Ray::new(new_point(0.0, 0.0, 5.0), new_vector(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [-6.0, -4.0]);
    }

    #[test]
    fn test_a_spheres_default_transformation() {
        let s = Sphere::new(0);
        assert_approx_eq!(s.transform, &IDENTITY4);
    }

    #[test]
    fn test_changing_a_spheres_transformation() {
        let mut s = Sphere::new(0);
        let t = translation(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_approx_eq!(s.transform, &t);
    }

    #[test]
    fn test_intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new(0);
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [3.0, 7.0]);
    }

    #[test]
    fn test_intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new(0);
        s.set_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, []);
    }
}
