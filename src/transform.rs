use std::ops::Mul;
use crate::approx_eq::ApproxEq;
use crate::matrix::{Matrix, IDENTITY_MATRIX};
use crate::point::Point;
use crate::vector::{Vector, ZERO};

#[derive(Copy, Clone, Debug)]
pub struct Affine {
    pub transform: Matrix,
    translate: Vector,
}

pub const IDENTITY_AFFINE: Affine = Affine {
    transform: IDENTITY_MATRIX,
    translate: ZERO,
};

impl Affine {
    pub fn new(transform: Matrix, translate: Vector) -> Self {
        Self {
            transform,
            translate,
        }
    }
    pub fn inverse(&self) -> Option<Self> {
        self.transform.inverse().map(|inv_trans| Self {
            transform: inv_trans,
            translate: -(inv_trans * &self.translate),
        })
    }
}

impl ApproxEq for Affine {
    fn approx_eq(&self, other: &Self) -> bool {
        self.transform.approx_eq(&other.transform) && self.translate.approx_eq(&other.translate)
    }
}

impl Mul<&Point> for &Affine {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        self.transform * rhs + &self.translate
    }
}

impl Mul<&Point> for Affine {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&Vector> for &Affine {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        self.transform * rhs
    }
}

impl Mul<&Vector> for Affine {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&Affine> for &Affine {
    type Output = Affine;

    fn mul(self, rhs: &Affine) -> Self::Output {
        Self::Output {
            transform: self.transform * &rhs.transform,
            translate: self * &rhs.translate
        }
    }
}

impl Mul<&Affine> for Affine {
    type Output = Affine;

    fn mul(self, rhs: &Affine) -> Self::Output {
        &self * rhs
    }
}

pub fn translation(x: f64, y: f64, z: f64) -> Affine {
    Affine::new(IDENTITY_MATRIX, Vector::new(x, y, z))
}

pub fn scaling(x: f64, y: f64, z: f64) -> Affine {
    Affine::new(
        Matrix::new([[x, 0.0, 0.0], [0.0, y, 0.0], [0.0, 0.0, z]]),
        ZERO,
    )
}

pub fn rotation_x(r: f64) -> Affine {
    let cr = r.cos();
    let sr = r.sin();
    Affine::new(
        Matrix::new([[1.0, 0.0, 0.0], [0.0, cr, -sr], [0.0, sr, cr]]),
        ZERO,
    )
}

pub fn rotation_y(r: f64) -> Affine {
    let cr = r.cos();
    let sr = r.sin();
    Affine::new(
        Matrix::new([[cr, 0.0, sr], [0.0, 0.0, 0.0], [-sr, 0.0, cr]]),
        ZERO,
    )
}

pub fn rotation_z(r: f64) -> Affine {
    let cr = r.cos();
    let sr = r.sin();
    Affine::new(
        Matrix::new([[cr, -sr, 0.0], [sr, cr, 0.0], [0.0, 0.0, 1.0]]),
        ZERO,
    )
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Affine {
    Affine::new(
        Matrix::new([[1.0, xy, xz], [yx, 1.0, yz], [zx, zy, 1.0]]),
        ZERO,
    )
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::f64::consts::PI;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};

    #[test]
    fn test_multiplying_by_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_approx_eq!(transform * &p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_approx_eq!(inv * &p, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        assert_approx_eq!(transform * &v, v);
    }

    #[test]
    fn test_a_scaling_matrix_applied_to_a_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert_approx_eq!(transform * &p, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_a_scaling_matrix_applied_to_a_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_approx_eq!(transform * &v, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_approx_eq!(inv * &v, Vector::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_reflection_is_scaling_by_a_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotating_a_point_around_the_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_approx_eq!(
            half_quarter * &p,
            Point::new(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0)
        );
        assert_approx_eq!(full_quarter * &p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_rotating_a_point_around_the_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_approx_eq!(
            half_quarter * &p,
            Point::new(2f64.sqrt() / 2.0, 0.0, 2f64.sqrt() / 2.0)
        );
        assert_approx_eq!(full_quarter * &p, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotating_a_point_around_the_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_approx_eq!(
            half_quarter * &p,
            Point::new(-2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0.0)
        );
        assert_approx_eq!(full_quarter * &p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, Point::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, Point::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, Point::new(2.0, 3.0, 7.0));
    }
}
