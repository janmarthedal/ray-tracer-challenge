use std::f64::consts::PI;

use crate::approx_eq::{assert_approx_eq, ApproxEq};
use crate::matrix::Matrix;
use crate::tuple::{new_point, new_vector};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix<4> {
    Matrix::<4>::new([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix<4> {
    Matrix::<4>::new([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_x(r: f64) -> Matrix<4> {
    let cr = r.cos();
    let sr = r.sin();
    Matrix::<4>::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cr, -sr, 0.0],
        [0.0, sr, cr, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_y(r: f64) -> Matrix<4> {
    let cr = r.cos();
    let sr = r.sin();
    Matrix::<4>::new([
        [cr, 0.0, sr, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [-sr, 0.0, cr, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_z(r: f64) -> Matrix<4> {
    let cr = r.cos();
    let sr = r.sin();
    Matrix::<4>::new([
        [cr, -sr, 0.0, 0.0],
        [sr, cr, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix<4> {
    Matrix::<4>::new([
        [1.0, xy, xz, 0.0],
        [yx, 1.0, yz, 0.0],
        [zx, zy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_multiplying_by_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = new_point(-3.0, 4.0, 5.0);
        assert_approx_eq!(transform * &p, new_point(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = new_point(-3.0, 4.0, 5.0);
        assert_approx_eq!(inv * &p, new_point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = new_vector(-3.0, 4.0, 5.0);
        assert_approx_eq!(transform * &v, v);
    }

    #[test]
    fn test_a_scaling_matrix_applied_to_a_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = new_point(-4.0, 6.0, 8.0);
        assert_approx_eq!(transform * &p, new_point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_a_scaling_matrix_applied_to_a_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = new_vector(-4.0, 6.0, 8.0);
        assert_approx_eq!(transform * &v, new_vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = new_vector(-4.0, 6.0, 8.0);
        assert_approx_eq!(inv * &v, new_vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_reflection_is_scaling_by_a_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, new_point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotating_a_point_around_the_x_axis() {
        let p = new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_approx_eq!(half_quarter * &p, new_point(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0));
        assert_approx_eq!(full_quarter * &p, new_point(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_rotating_a_point_around_the_y_axis() {
        let p = new_point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_approx_eq!(half_quarter * &p, new_point(2f64.sqrt() / 2.0, 0.0, 2f64.sqrt() / 2.0));
        assert_approx_eq!(full_quarter * &p, new_point(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotating_a_point_around_the_z_axis() {
        let p = new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_approx_eq!(half_quarter * &p, new_point(-2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0.0));
        assert_approx_eq!(full_quarter * &p, new_point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, new_point(5.0, 3.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, new_point(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, new_point(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, new_point(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, new_point(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_approx_eq!(transform * &p, new_point(2.0, 3.0, 7.0));
    }
}
