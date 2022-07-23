use crate::material::{Material, DEFAULT_MATERIAL};
use crate::point::{Point, ORIGIN};
use crate::ray::Ray;
use crate::transform::{Affine, IDENTITY_AFFINE};
use crate::vector::Vector;
use crate::world::Object;

pub struct Sphere {
    transform: Affine,
    material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transform: IDENTITY_AFFINE,
            material: DEFAULT_MATERIAL,
        }
    }
    pub fn set_transform(&self, transform: Affine) -> Self {
        Self { transform, ..*self }
    }
    pub fn set_material(&self, material: Material) -> Self {
        Self { material, ..*self }
    }
}

impl Object for Sphere {
    fn get_material(&self) -> &Material {
        &self.material
    }
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let ray = ray.transform(&self.transform.inverse().unwrap());
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
    fn normal_at(&self, world_point: &Point) -> Vector {
        let inverse_transform = self.transform.inverse().unwrap();
        let object_point = inverse_transform * world_point;
        let object_normal = object_point - &ORIGIN;
        let world_normal = inverse_transform.get_transform().transpose() * &object_normal;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::transform::{rotation_z, scaling, translation};

    #[test]
    fn test_a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [4.0, 6.0]);
    }

    #[test]
    fn test_a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [5.0, 5.0]);
    }

    #[test]
    fn test_a_ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, []);
    }

    #[test]
    fn test_a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [-1.0, 1.0]);
    }

    #[test]
    fn test_a_ray_is_behind_a_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [-6.0, -4.0]);
    }

    #[test]
    fn test_a_spheres_default_transformation() {
        let s = Sphere::new();
        assert_approx_eq!(s.transform, &IDENTITY_AFFINE);
    }

    #[test]
    fn test_changing_a_spheres_transformation() {
        let t = translation(2.0, 3.0, 4.0);
        let s = Sphere::new().set_transform(t);
        assert_approx_eq!(s.transform, &t);
    }

    #[test]
    fn test_intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new().set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [3.0, 7.0]);
    }

    #[test]
    fn test_intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new().set_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, []);
    }

    #[test]
    fn test_the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Point::new(1.0, 0.0, 0.0));
        assert_approx_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Point::new(0.0, 1.0, 0.0));
        assert_approx_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Point::new(0.0, 0.0, 1.0));
        assert_approx_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let c = 3f64.sqrt() / 3.0;
        let n = s.normal_at(&Point::new(c, c, c));
        assert_approx_eq!(n, Vector::new(c, c, c));
    }

    #[test]
    fn test_computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::new().set_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&Point::new(0.0, 1.70711, -0.70711));
        assert_approx_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn test_computing_the_normal_on_a_transformed_sphere() {
        let s = Sphere::new()
            .set_transform(scaling(1.0, 0.5, 1.0) * &rotation_z(std::f64::consts::PI / 5.0));
        let n = s.normal_at(&Point::new(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0));
        assert_approx_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
