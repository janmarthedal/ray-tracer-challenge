pub trait ApproxEq {
    fn approx_eq(self, other: &Self) -> bool;
}

const EPSILON: f64 = 0.00001;

impl ApproxEq for f64 {
    fn approx_eq(self, other: &Self) -> bool {
        (self - other).abs() < EPSILON
    }
}

macro_rules! assert_approx_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val).approx_eq(right_val) {
                    panic!("Not approx_eq, left: {:?}, right: {:?}", left_val, right_val);
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val).approx_eq(right_val) {
                    panic!("Not approx_eq, left: {:?}, right: {:?}", left_val, right_val);
                }
            }
        }
    };
}

pub(crate) use assert_approx_eq;
