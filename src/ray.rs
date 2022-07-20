use crate::transform::Affine;
use crate::point::Point;
use crate::vector::Vector;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }
    pub fn position(&self, t: f64) -> Point {
        self.origin + &(&self.direction * t)
    }
    pub fn transform(&self, trans: &Affine) -> Self {
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
    
    #[test]
    fn test_computing_a_point_from_a_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_approx_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_translating_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert_approx_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
        assert_approx_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_scaling_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);
        assert_approx_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
        assert_approx_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
    }
}
