pub trait ApproxEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn approx_eq(&self, other: &Rhs) -> bool;
}

const EPSILON: f64 = 0.00001;

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &f64) -> bool {
        (self - other).abs() < EPSILON
    }
}

impl<T: ApproxEq> ApproxEq for [T] {
    fn approx_eq(&self, other: &[T]) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a.approx_eq(b))
    }
}

impl<T: ApproxEq> ApproxEq for Vec<T> {
    fn approx_eq(&self, other: &Vec<T>) -> bool {
        self[..].approx_eq(&other[..])
    }
}

impl<T: ApproxEq, const N: usize> ApproxEq<[T; N]> for Vec<T> {
    fn approx_eq(&self, other: &[T; N]) -> bool {
        self[..].approx_eq(&other[..])
    }
}

#[cfg(test)]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val).approx_eq(right_val) {
                    panic!(
                        "Not approx_eq, left: {:?}, right: {:?}",
                        left_val, right_val
                    );
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val).approx_eq(right_val) {
                    panic!(
                        "Not approx_eq, left: {:?}, right: {:?}",
                        left_val, right_val
                    );
                }
            }
        }
    };
}

#[cfg(test)]
pub(crate) use assert_approx_eq;
