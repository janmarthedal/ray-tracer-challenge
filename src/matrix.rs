use crate::approx_eq::ApproxEq;
use crate::tuple::Tuple;
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub struct Matrix<const N: usize> {
    elems: [[f64; N]; N],
}

impl<const N: usize> Matrix<N> {
    pub fn new(elems: [[f64; N]; N]) -> Self {
        Self { elems }
    }
    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.elems[i][j]
    }
    pub fn transpose(&self) -> Self {
        let mut elems = [[0f64; N]; N];
        for i in 0..N {
            for j in 0..N {
                elems[i][j] = self.elems[j][i];
            }
        }
        Self { elems }
    }
}

impl<const N: usize> ApproxEq for Matrix<N> {
    fn approx_eq(self, other: &Self) -> bool {
        self.elems
            .iter()
            .zip(other.elems.iter())
            .all(|(row1, row2)| {
                row1.iter()
                    .zip(row2.iter())
                    .all(|(e1, e2)| e1.approx_eq(e2))
            })
    }
}

impl<const N: usize> Mul<&Matrix<N>> for Matrix<N> {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut elems = [[0f64; N]; N];
        for i in 0..N {
            for j in 0..N {
                let mut v = 0f64;
                for k in 0..N {
                    v += self.elems[i][k] * rhs.elems[k][j];
                }
                elems[i][j] = v;
            }
        }
        Self { elems }
    }
}

impl Mul<&Tuple> for Matrix<4> {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Self::Output {
        Tuple::new(
            self.elems[0][0] * rhs.x
                + self.elems[0][1] * rhs.y
                + self.elems[0][2] * rhs.z
                + self.elems[0][3] * rhs.w,
            self.elems[1][0] * rhs.x
                + self.elems[1][1] * rhs.y
                + self.elems[1][2] * rhs.z
                + self.elems[1][3] * rhs.w,
            self.elems[2][0] * rhs.x
                + self.elems[2][1] * rhs.y
                + self.elems[2][2] * rhs.z
                + self.elems[2][3] * rhs.w,
            self.elems[3][0] * rhs.x
                + self.elems[3][1] * rhs.y
                + self.elems[3][2] * rhs.z
                + self.elems[3][3] * rhs.w,
        )
    }
}

impl Matrix<2> {
    fn determinant(&self) -> f64 {
        self.elems[0][0] * self.elems[1][1] - self.elems[1][0] * self.elems[0][1]
    }
}

impl Matrix<3> {
    fn submatrix(&self, i: usize, j: usize) -> Matrix<2> {
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
        Matrix::<2> { elems }
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
}

impl Matrix<4> {
    fn submatrix(&self, i: usize, j: usize) -> Matrix<3> {
        let mut elems = [[0f64; 3]; 3];
        let mut j2 = 0;
        for j1 in 0..4 {
            if j1 != j {
                let mut i2 = 0;
                for i1 in 0..4 {
                    if i1 != i {
                        elems[i2][j2] = self.elems[i1][j1];
                        i2 += 1;
                    }
                }
                j2 += 1;
            }
        }
        Matrix::<3> { elems }
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
            + self.elems[0][3] * self.cofactor(0, 3)
    }
    fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.approx_eq(&0.0) {
            return None;
        }
        let mut elems = [[0f64; 4]; 4];
        for j in 0..4 {
            for i in 0..4 {
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
    fn test_constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m.at(0, 0), 1.0);
        assert_eq!(m.at(0, 3), 4.0);
        assert_eq!(m.at(1, 0), 5.5);
        assert_eq!(m.at(1, 2), 7.5);
        assert_eq!(m.at(2, 2), 11.0);
        assert_eq!(m.at(3, 0), 13.5);
        assert_eq!(m.at(3, 2), 15.5);
    }

    #[test]
    fn test_a_2x2_matrix_ought_to_be_representable() {
        let m: Matrix<2> = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);
        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(0, 1), 5.0);
        assert_eq!(m.at(1, 0), 1.0);
        assert_eq!(m.at(1, 1), -2.0);
    }

    #[test]
    fn test_a_3x3_matrix_ought_to_be_representable() {
        let m: Matrix<3> = Matrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(1, 1), -2.0);
        assert_eq!(m.at(2, 2), 1.0);
    }

    #[test]
    fn test_matrix_equality_with_identical_matrices() {
        let m1: Matrix<4> = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix<4> = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert_approx_eq!(m1, m2);
    }

    #[test]
    fn test_matrix_equality_with_different_matrices() {
        let m1: Matrix<4> = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix<4> = Matrix::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert!(!m1.approx_eq(&m2));
    }

    #[test]
    fn test_multiplying_two_matrices() {
        let a: Matrix<4> = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b: Matrix<4> = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        assert_approx_eq!(
            a * &b,
            Matrix::<4>::new([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        )
    }

    #[test]
    fn test_a_matrix_multiplied_by_a_tuple() {
        let a: Matrix<4> = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        assert_approx_eq!(a * &b, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn test_multiplying_a_matrix_by_the_identity_matrix() {
        let a: Matrix<4> = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        let identity_matrix: Matrix<4> = Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_approx_eq!(a * &identity_matrix, a)
    }

    #[test]
    fn test_multiplying_the_identity_matrix_by_a_tuple() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let identity_matrix: Matrix<4> = Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_approx_eq!(identity_matrix * &a, a);
    }

    #[test]
    fn test_transposing_a_matrix() {
        let a: Matrix<4> = Matrix::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let tra: Matrix<4> = Matrix::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_approx_eq!(a.transpose(), tra);
    }

    #[test]
    fn test_transposing_the_identity_matrix() {
        let identity_matrix: Matrix<4> = Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_approx_eq!(identity_matrix.transpose(), identity_matrix);
    }

    #[test]
    fn test_calculating_the_determinant_of_a_2x2_matrix() {
        let a: Matrix<2> = Matrix::new([[1.0, 5.0], [-3.0, 2.0]]);
        assert_approx_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn test_a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a: Matrix<3> = Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        assert_approx_eq!(
            a.submatrix(0, 2),
            Matrix::<2>::new([[-3.0, 2.0], [0.0, 6.0]])
        )
    }

    #[test]
    fn test_a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let a: Matrix<4> = Matrix::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        assert_approx_eq!(
            a.submatrix(2, 1),
            Matrix::<3>::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]])
        )
    }

    #[test]
    fn test_calculating_a_minor_of_a_3x3_matrix() {
        let a: Matrix<3> = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let b = a.submatrix(1, 0);
        assert_approx_eq!(b.determinant(), 25.0);
        assert_approx_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn test_calculating_a_cofactor_of_a_3x3_matrix() {
        let a: Matrix<3> = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_approx_eq!(a.minor(0, 0), -12.0);
        assert_approx_eq!(a.cofactor(0, 0), -12.0);
        assert_approx_eq!(a.minor(1, 0), 25.0);
        assert_approx_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_calculating_the_determinant_of_a_3x3_matrix() {
        let a: Matrix<3> = Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_approx_eq!(a.cofactor(0, 0), 56.0);
        assert_approx_eq!(a.cofactor(0, 1), 12.0);
        assert_approx_eq!(a.cofactor(0, 2), -46.0);
        assert_approx_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn test_calculating_the_determinant_of_a_4x4_matrix() {
        let a: Matrix<4> = Matrix::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_approx_eq!(a.cofactor(0, 0), 690.0);
        assert_approx_eq!(a.cofactor(0, 1), 447.0);
        assert_approx_eq!(a.cofactor(0, 2), 210.0);
        assert_approx_eq!(a.cofactor(0, 3), 51.0);
        assert_approx_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn test_calculating_the_inverse_of_a_matrix() {
        let a: Matrix<4> = Matrix::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse().unwrap();
        assert_approx_eq!(a.determinant(), 532.0);
        assert_approx_eq!(a.cofactor(2, 3), -160.0);
        assert_approx_eq!(b.at(3, 2), -160.0 / 532.0);
        assert_approx_eq!(a.cofactor(3, 2), 105.0);
        assert_approx_eq!(b.at(2, 3), 105.0 / 532.0);
        assert_approx_eq!(
            b,
            Matrix::<4>::new([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639]
            ])
        );
    }

    #[test]
    fn test_calculating_the_inverse_of_another_matrix() {
        let a: Matrix<4> = Matrix::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        assert_approx_eq!(
            a.inverse().unwrap(),
            Matrix::<4>::new([
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308]
            ])
        );
    }

    #[test]
    fn test_calculating_the_inverse_of_third_matrix() {
        let a: Matrix<4> = Matrix::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        assert_approx_eq!(
            a.inverse().unwrap(),
            Matrix::<4>::new([
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333]
            ])
        );
    }

    #[test]
    fn test_multiplying_a_product_by_its_inverse() {
        let a: Matrix<4> = Matrix::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b: Matrix<4> = Matrix::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a * &b;
        assert_approx_eq!(c * &b.inverse().unwrap(), a);
    }
}
