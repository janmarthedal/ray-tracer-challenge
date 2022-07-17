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
}
