use crate::approx_eq::ApproxEq;

pub struct Matrix2 {
    elems: [f64; 4],
}

impl Matrix2 {
    pub fn new(elems: [f64; 4]) -> Self {
        Self { elems }
    }
    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.elems[2 * i + j]
    }
}

pub struct Matrix3 {
    elems: [f64; 9],
}

impl Matrix3 {
    pub fn new(elems: [f64; 9]) -> Self {
        Self { elems }
    }
    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.elems[3 * i + j]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
    elems: [f64; 16],
}

impl Matrix4 {
    pub fn new(elems: [f64; 16]) -> Self {
        Self { elems }
    }
    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.elems[4 * i + j]
    }
}

impl ApproxEq for Matrix4 {
    fn approx_eq(self, other: &Self) -> bool {
        /* for k in 0..16 {
            if !self.elems[k].approx_eq(&other.elems[k]) {
                return false;
            }
        }
        true */
        self.elems
            .iter()
            .zip(other.elems.iter())
            .all(|(a, b)| a.approx_eq(b))
    }
}

#[cfg(test)]
mod tests {

    use crate::approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn test_constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
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
        let m: Matrix2 = Matrix2::new([-3.0, 5.0, 1.0, -2.0]);
        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(0, 1), 5.0);
        assert_eq!(m.at(1, 0), 1.0);
        assert_eq!(m.at(1, 1), -2.0);
    }

    #[test]
    fn test_a_3x3_matrix_ought_to_be_representable() {
        let m: Matrix3 = Matrix3::new([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(1, 1), -2.0);
        assert_eq!(m.at(2, 2), 1.0);
    }

    #[test]
    fn test_matrix_equality_with_identical_matrices() {
        let m1 = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let m2 = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        assert_approx_eq!(m1, m2);
    }

    #[test]
    fn test_matrix_equality_with_different_matrices() {
        let m1 = Matrix4::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let m2 = Matrix4::new([
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ]);
        assert!(!m1.approx_eq(&m2));
    }
}
