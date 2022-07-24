use crate::material::{Material, DEFAULT_MATERIAL};
use crate::point::Point;
use crate::ray::Ray;
use crate::transform::{Affine, IDENTITY_AFFINE};
use crate::vector::Vector;

pub trait LocalShape {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64>;
    fn local_normal_at(&self, p: &Point) -> Vector;
}

pub struct Shape<'a> {
    inverse_transform: Affine,
    material: Material<'a>,
    local_shape: Box<dyn LocalShape + 'a>,
}

impl<'a> Shape<'a> {
    pub fn new(local_shape: impl LocalShape + 'a) -> Self {
        Self {
            inverse_transform: IDENTITY_AFFINE,
            material: DEFAULT_MATERIAL,
            local_shape: Box::new(local_shape),
        }
    }
    pub fn set_transform(self, transform: Affine) -> Self {
        Self {
            inverse_transform: transform.inverse().unwrap(),
            ..self
        }
    }
    pub fn set_material(self, material: Material<'a>) -> Self {
        Self { material, ..self }
    }
    pub fn get_material(&self) -> &Material {
        &self.material
    }
    pub fn get_inverse_transform(&self) -> &Affine {
        &self.inverse_transform
    }
    pub fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let ray = ray.transform(&self.inverse_transform);
        self.local_shape.local_intersect(&ray)
    }
    pub fn normal_at(&self, point: &Point) -> Vector {
        let local_point = self.inverse_transform * point;
        let local_normal = self.local_shape.local_normal_at(&local_point);
        let world_normal = self.inverse_transform.get_transform().transpose() * &local_normal;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::point::ORIGIN;
    use crate::transform::{rotation_z, scaling, translation};

    struct TestShape {}

    impl TestShape {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl LocalShape for TestShape {
        fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
            // hack to get the local ray values out
            vec![
                ray.origin.x,
                ray.origin.y,
                ray.origin.z,
                ray.direction.x,
                ray.direction.y,
                ray.direction.z,
            ]
        }
        fn local_normal_at(&self, object_point: &Point) -> Vector {
            object_point - &ORIGIN
        }
    }

    #[test]
    fn test_a_shapes_default_transformation() {
        let s = Shape::new(TestShape::new());
        assert_approx_eq!(s.inverse_transform, &IDENTITY_AFFINE);
    }

    #[test]
    fn test_changing_a_shapes_transformation() {
        let t = translation(2.0, 3.0, 4.0);
        let s = Shape::new(TestShape::new()).set_transform(t);
        assert_approx_eq!(s.inverse_transform, &t.inverse().unwrap());
    }

    #[test]
    fn test_intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::new(TestShape::new()).set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [0.0, 0.0, -2.5, 0.0, 0.0, 0.5]);
    }

    #[test]
    fn test_intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::new(TestShape::new()).set_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [-5.0, 0.0, -5.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_computing_the_normal_on_a_translated_shape() {
        let s = Shape::new(TestShape::new()).set_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&Point::new(0.0, 1.70711, -0.70711));
        assert_approx_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn test_computing_the_normal_on_a_transformed_sphere() {
        let s = Shape::new(TestShape::new())
            .set_transform(scaling(1.0, 0.5, 1.0) * &rotation_z(std::f64::consts::PI / 5.0));
        let n = s.normal_at(&Point::new(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0));
        assert_approx_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
