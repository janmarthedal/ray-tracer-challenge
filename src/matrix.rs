use crate::approx_eq::ApproxEq;
use crate::point::Point;
use crate::vector::Vector;
use std::ops::Mul;

pub const IDENTITY_MATRIX: Matrix = Matrix {
    elems: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
};

#[derive(Copy, Clone, Debug)]
pub struct Matrix {
    elems: [[f64; 3]; 3],
}

impl Matrix {
    pub fn new(elems: [[f64; 3]; 3]) -> Self {
        Self { elems }
    }
    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.elems[i][j]
    }
    pub fn transpose(&self) -> Self {
        let mut elems = [[0f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                elems[i][j] = self.elems[j][i];
            }
        }
        Self { elems }
    }
}

impl ApproxEq for Matrix {
    fn approx_eq(&self, other: &Self) -> bool {
        self.elems[0].approx_eq(&other.elems[0])
            && self.elems[1].approx_eq(&other.elems[1])
            && self.elems[2].approx_eq(&other.elems[2])
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut elems = [[0f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                elems[i][j] = self.elems[i][0] * rhs.elems[0][j]
                    + self.elems[i][1] * rhs.elems[1][j]
                    + self.elems[i][2] * rhs.elems[2][j];
            }
        }
        Self { elems }
    }
}

impl Mul<&Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector::new(
            self.elems[0][0] * rhs.x + self.elems[0][1] * rhs.y + self.elems[0][2] * rhs.z,
            self.elems[1][0] * rhs.x + self.elems[1][1] * rhs.y + self.elems[1][2] * rhs.z,
            self.elems[2][0] * rhs.x + self.elems[2][1] * rhs.y + self.elems[2][2] * rhs.z,
        )
    }
}

impl Mul<&Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&Point> for &Matrix {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::new(
            self.elems[0][0] * rhs.x + self.elems[0][1] * rhs.y + self.elems[0][2] * rhs.z,
            self.elems[1][0] * rhs.x + self.elems[1][1] * rhs.y + self.elems[1][2] * rhs.z,
            self.elems[2][0] * rhs.x + self.elems[2][1] * rhs.y + self.elems[2][2] * rhs.z,
        )
    }
}

impl Mul<&Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        &self * rhs
    }
}

#[derive(Debug)]
struct Matrix2 {
    elems: [[f64; 2]; 2],
}

impl Matrix2 {
    fn determinant(&self) -> f64 {
        self.elems[0][0] * self.elems[1][1] - self.elems[1][0] * self.elems[0][1]
    }
}

impl ApproxEq for Matrix2 {
    fn approx_eq(&self, other: &Self) -> bool {
        self.elems[0].approx_eq(&other.elems[0]) && self.elems[1].approx_eq(&other.elems[1])
    }
}

impl Matrix {
    fn submatrix(&self, i: usize, j: usize) -> Matrix2 {
        let mut elems = [[0f64; 2]; 2];
        let mut j2 = 0;
        for j1 in 0..3 {
            if j1 != j {
                let mut i2 = 0;
                for i1 in 0..3 {
                    if i1 != i {
                        elems[i2][j2] = self.elems[i1][j1];
                        i2 += 1;
                    }
                }
                j2 += 1;
            }
        }
        Matrix2 { elems }
    }
    fn minor(&self, i: usize, j: usize) -> f64 {
        self.submatrix(i, j).determinant()
    }
    fn cofactor(&self, i: usize, j: usize) -> f64 {
        let m = self.minor(i, j);
        if (i + j) % 2 == 0 {
            m
        } else {
            -m
        }
    }
    fn determinant(&self) -> f64 {
        self.elems[0][0] * self.cofactor(0, 0)
            + self.elems[0][1] * self.cofactor(0, 1)
            + self.elems[0][2] * self.cofactor(0, 2)
    }
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.approx_eq(&0.0) {
            return None;
        }
        let mut elems = [[0f64; 3]; 3];
        for j in 0..3 {
            for i in 0..3 {
                let c = self.cofactor(i, j);
                elems[j][i] = c / det;
            }
        }
        Some(Self { elems })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::assert_approx_eq;

    #[test]
    fn test_matrix_equality_with_identical_matrices() {
        let m1 = Matrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]]);
        let m2 = Matrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]]);
        assert_approx_eq!(m1, m2);
    }

    #[test]
    fn test_matrix_equality_with_different_matrices() {
        let m1 = Matrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]]);
        let m2 = Matrix::new([[2.0, 3.0, 4.0], [6.0, 7.0, 8.0], [8.0, 7.0, 6.0]]);
        assert!(!m1.approx_eq(&m2));
    }

    #[test]
    fn test_multiplying_two_matrices() {
        let a = Matrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 8.0, 7.0]]);
        let b = Matrix::new([[-2.0, 1.0, 2.0], [3.0, 2.0, 1.0], [4.0, 3.0, 6.0]]);
        assert_approx_eq!(
            a * &b,
            Matrix::new([[16.0, 14.0, 22.0], [36.0, 38.0, 58.0], [34.0, 46.0, 68.0]])
        )
    }

    #[test]
    fn test_a_matrix_multiplied_by_a_vector() {
        let a = Matrix::new([[1.0, 2.0, 3.0], [2.0, 4.0, 4.0], [8.0, 6.0, 4.0]]);
        let b = Vector::new(1.0, 2.0, 3.0);
        assert_approx_eq!(a * &b, Vector::new(14.0, 22.0, 32.0));
    }

    #[test]
    fn test_multiplying_a_matrix_by_the_identity_matrix() {
        let a = Matrix::new([[0.0, 1.0, 2.0], [1.0, 2.0, 4.0], [2.0, 4.0, 8.0]]);
        assert_approx_eq!(a * &IDENTITY_MATRIX, a)
    }

    #[test]
    fn test_multiplying_the_identity_matrix_by_a_vector() {
        let a = Vector::new(1.0, 2.0, 3.0);
        assert_approx_eq!(IDENTITY_MATRIX * &a, a);
    }

    #[test]
    fn test_transposing_a_matrix() {
        let a = Matrix::new([[0.0, 9.0, 3.0], [9.0, 8.0, 0.0], [1.0, 8.0, 5.0]]);
        let tra = Matrix::new([[0.0, 9.0, 1.0], [9.0, 8.0, 8.0], [3.0, 0.0, 5.0]]);
        assert_approx_eq!(a.transpose(), tra);
    }

    #[test]
    fn test_transposing_the_identity_matrix() {
        assert_approx_eq!(IDENTITY_MATRIX.transpose(), IDENTITY_MATRIX);
    }

    #[test]
    fn test_calculating_the_determinant_of_a_2x2_matrix() {
        let a = Matrix2 {
            elems: [[1.0, 5.0], [-3.0, 2.0]],
        };
        assert_approx_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn test_a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a = Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        assert_approx_eq!(
            a.submatrix(0, 2),
            Matrix2 {
                elems: [[-3.0, 2.0], [0.0, 6.0]]
            }
        )
    }

    #[test]
    fn test_calculating_a_minor_of_a_3x3_matrix() {
        let a = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let b = a.submatrix(1, 0);
        assert_approx_eq!(b.determinant(), 25.0);
        assert_approx_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn test_calculating_a_cofactor_of_a_3x3_matrix() {
        let a = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_approx_eq!(a.minor(0, 0), -12.0);
        assert_approx_eq!(a.cofactor(0, 0), -12.0);
        assert_approx_eq!(a.minor(1, 0), 25.0);
        assert_approx_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_calculating_the_determinant_of_a_3x3_matrix() {
        let a = Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_approx_eq!(a.cofactor(0, 0), 56.0);
        assert_approx_eq!(a.cofactor(0, 1), 12.0);
        assert_approx_eq!(a.cofactor(0, 2), -46.0);
        assert_approx_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn test_calculating_the_inverse_of_a_matrix() {
        let a = Matrix::new([[-5.0, 2.0, 6.0], [1.0, -5.0, 1.0], [7.0, 7.0, -6.0]]);
        let b = a.inverse().unwrap();
        assert_approx_eq!(a * &b, IDENTITY_MATRIX);
    }

    #[test]
    fn test_calculating_the_inverse_of_another_matrix() {
        let a = Matrix::new([[8.0, -5.0, 9.0], [7.0, 5.0, 6.0], [-6.0, 0.0, 9.0]]);
        let b = a.inverse().unwrap();
        assert_approx_eq!(a * &b, IDENTITY_MATRIX);
    }

    #[test]
    fn test_calculating_the_inverse_of_third_matrix() {
        let a = Matrix::new([[9.0, 3.0, 0.0], [-5.0, -2.0, -6.0], [-4.0, 9.0, 6.0]]);
        let b = a.inverse().unwrap();
        assert_approx_eq!(a * &b, IDENTITY_MATRIX);
    }

    #[test]
    fn test_multiplying_a_product_by_its_inverse() {
        let a = Matrix::new([[3.0, -9.0, 7.0], [3.0, -8.0, 2.0], [-4.0, 4.0, 4.0]]);
        let b = Matrix::new([[8.0, 2.0, 2.0], [3.0, -1.0, 7.0], [7.0, 0.0, 5.0]]);
        let c = a * &b;
        assert_approx_eq!(c * &b.inverse().unwrap(), a);
    }
}
