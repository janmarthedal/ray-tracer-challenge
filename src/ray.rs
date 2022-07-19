use crate::tuple::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }
    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {

    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::tuple::{new_point, new_vector};
    use super::*;

    #[test]
    fn test_computing_a_point_from_a_distance() {
        let r = Ray::new(new_point(2.0, 3.0, 4.0), new_vector(1.0, 0.0, 0.0));
        assert_approx_eq!(r.position(0.0), new_point(2.0, 3.0, 4.0));
    }
}