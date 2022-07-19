use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::approx_eq::ApproxEq;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self / self.magnitude()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, b: &Self) -> Self {
        // assumes self and b are vectors
        Self {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
            w: 0.0,
        }
    }
}

impl ApproxEq for Tuple {
    fn approx_eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x)
        && self.y.approx_eq(&other.y)
        && self.z.approx_eq(&other.z)
        && self.w.approx_eq(&other.w)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f64) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        Self::Output {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
            w: self * other.w,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, other: f64) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Div<f64> for &Tuple {
    type Output = Tuple;

    fn div(self, other: f64) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

pub fn new_point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::assert_approx_eq;

    #[test]
    fn test_new_point() {
        let p = new_point(4.0, -4.0, 3.0);
        assert_approx_eq!(p.x, 4.0);
        assert_approx_eq!(p.y, -4.0);
        assert_approx_eq!(p.z, 3.0);
        assert_approx_eq!(p.w, 1.0);
    }

    #[test]
    fn test_new_vector() {
        let p = new_vector(4.0, -4.0, 3.0);
        assert_approx_eq!(p.x, 4.0);
        assert_approx_eq!(p.y, -4.0);
        assert_approx_eq!(p.z, 3.0);
        assert_approx_eq!(p.w, 0.0);
    }

    #[test]
    fn test_adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_approx_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn test_subtracting_two_points() {
        let p1 = new_point(3.0, 2.0, 1.0);
        let p2 = new_point(5.0, 6.0, 7.0);
        assert_approx_eq!(p1 - p2, new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_a_vector_from_a_point() {
        let p = new_point(3.0, 2.0, 1.0);
        let v = new_vector(5.0, 6.0, 7.0);
        assert_approx_eq!(p - v, new_point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_two_vectors() {
        let v1 = new_vector(3.0, 2.0, 1.0);
        let v2 = new_vector(5.0, 6.0, 7.0);
        assert_approx_eq!(v1 - v2, new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_a_vector_from_the_zero_vector() {
        let zero = new_vector(0.0, 0.0, 0.0);
        let v = new_vector(1.0, -2.0, 3.0);
        assert_approx_eq!(zero - v, new_vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_approx_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_approx_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn test_multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_approx_eq!(0.5 * a, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_approx_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_the_magnitude_of_vector1() {
        let v = new_vector(1.0, 0.0, 0.0);
        assert_approx_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn test_the_magnitude_of_vector2() {
        let v = new_vector(0.0, 1.0, 0.0);
        assert_approx_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn test_the_magnitude_of_vector3() {
        let v = new_vector(0.0, 0.0, 1.0);
        assert_approx_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn test_the_magnitude_of_vector4() {
        let v = new_vector(1.0, 2.0, 3.0);
        assert_approx_eq!(v.magnitude(), (14f64).sqrt());
    }

    #[test]
    fn test_the_magnitude_of_vector5() {
        let v = new_vector(-1.0, -2.0, -3.0);
        assert_approx_eq!(v.magnitude(), (14f64).sqrt());
    }

    #[test]
    fn test_normalizing_vector1() {
        let v = new_vector(4.0, 0.0, 0.0);
        assert_approx_eq!(v.normalize(), new_vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_normalizing_vector2() {
        let v = new_vector(1.0, 2.0, 3.0);
        assert_approx_eq!(
            v.normalize(),
            new_vector(1.0 / 14f64.sqrt(), 2.0 / 14f64.sqrt(), 3.0 / 14f64.sqrt())
        );
    }

    #[test]
    fn test_the_magnitude_of_a_normalized_vector() {
        let v = new_vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_approx_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn test_the_dot_product_of_two_tuples() {
        let a = new_vector(1.0, 2.0, 3.0);
        let b = new_vector(2.0, 3.0, 4.0);
        assert_approx_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn test_the_cross_product_of_two_vectors() {
        let a = new_vector(1.0, 2.0, 3.0);
        let b = new_vector(2.0, 3.0, 4.0);
        assert_approx_eq!(a.cross(&b), new_vector(-1.0, 2.0, -1.0));
        assert_approx_eq!(b.cross(&a), new_vector(1.0, -2.0, 1.0));
    }
}
