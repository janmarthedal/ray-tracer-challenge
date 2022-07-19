use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }
    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
    pub fn transform(&self, trans: &Matrix<4>) -> Self {
        Self {
            origin: trans * &self.origin,
            direction: trans * &self.direction,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::transform::{translation, scaling};
    use crate::tuple::{new_point, new_vector};

    #[test]
    fn test_computing_a_point_from_a_distance() {
        let r = Ray::new(new_point(2.0, 3.0, 4.0), new_vector(1.0, 0.0, 0.0));
        assert_approx_eq!(r.position(0.0), new_point(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_translating_a_ray() {
        let r = Ray::new(new_point(1.0, 2.0, 3.0), new_vector(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert_approx_eq!(r2.origin, new_point(4.0, 6.0, 8.0));
        assert_approx_eq!(r2.direction, new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_scaling_a_ray() {
        let r = Ray::new(new_point(1.0, 2.0, 3.0), new_vector(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);
        assert_approx_eq!(r2.origin, new_point(2.0, 6.0, 12.0));
        assert_approx_eq!(r2.direction, new_vector(0.0, 3.0, 0.0));
    }
}
