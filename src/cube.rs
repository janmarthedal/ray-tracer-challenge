use crate::approx_eq::EPSILON;
use crate::point::{Point, ORIGIN};
use crate::ray::Ray;
use crate::shape::LocalShape;
use crate::vector::Vector;

pub struct Cube {}

impl Cube {
    pub fn new() -> Self {
        Self {}
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    // If `direction` is zero, `tmin` and `tmax` will be infinity (with
    // proper signs), which is handled later
    let tmin = tmin_numerator / direction;
    let tmax = tmax_numerator / direction;

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl LocalShape for Cube {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            vec![]
        } else {
            vec![tmin, tmax]
        }
    }
    fn local_normal_at(&self, point: &Point) -> Vector {
        let maxc = point.x.abs().max(point.y.abs()).max(point.z.abs());

        if maxc == point.x.abs() {
            Vector::new(point.x, 0.0, 0.0)
        } else if maxc == point.y.abs() {
            Vector::new(0.0, point.y, 0.0)
        } else {
            Vector::new(0.0, 0.0, point.z)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};

    #[test]
    fn test_a_ray_intersects_a_cube() {
        let cube = Cube::new();
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(5.0, 0.5, 0.0),
                Vector::new(-1.0, 0.0, 0.0)
            )),
            [4.0, 6.0]
        );
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(-5.0, 0.5, 0.0),
                Vector::new(1.0, 0.0, 0.0)
            )),
            [4.0, 6.0]
        );
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(0.5, 5.0, 0.0),
                Vector::new(0.0, -1.0, 0.0)
            )),
            [4.0, 6.0]
        );
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(0.5, -5.0, 0.0),
                Vector::new(0.0, 1.0, 0.0)
            )),
            [4.0, 6.0]
        );
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(0.5, 0.0, 5.0),
                Vector::new(0.0, 0.0, -1.0)
            )),
            [4.0, 6.0]
        );
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0)
            )),
            [4.0, 6.0]
        );
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(0.0, 0.5, 0.0),
                Vector::new(0.0, 0.0, 1.0)
            )),
            [-1.0, 1.0]
        );
    }

    #[test]
    fn test_a_ray_misses_a_cube() {
        let cube = Cube::new();
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(-2.0, 0.0, 0.0),
                Vector::new(0.2673, 0.5345, 0.8018)
            )),
            []
        );
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(0.0, -2.0, 0.0),
                Vector::new(0.8018, 0.2673, 0.5345)
            )),
            []
        );
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(0.0, 0.0, -2.0),
                Vector::new(0.5345, 0.8018, 0.2673)
            )),
            []
        );
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(2.0, 0.0, 2.0),
                Vector::new(0.0, 0.0, -1.0)
            )),
            []
        );
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(0.0, 2.0, 2.0),
                Vector::new(0.0, -1.0, 0.0)
            )),
            []
        );
        assert_approx_eq!(
            cube.local_intersect(&Ray::new(
                Point::new(2.0, 2.0, 0.0),
                Vector::new(-1.0, 0.0, 0.0)
            )),
            []
        );
    }

    #[test]
    fn test_the_normal_on_the_surface_of_a_cube() {
        let c = Cube::new();
        assert_approx_eq!(
            c.local_normal_at(&Point::new(1.0, 0.5, -0.8)),
            &Vector::new(1.0, 0.0, 0.0)
        );
        assert_approx_eq!(
            c.local_normal_at(&Point::new(-1.0, -0.2, 0.9)),
            &Vector::new(-1.0, 0.0, 0.0)
        );
        assert_approx_eq!(
            c.local_normal_at(&Point::new(-0.4, 1.0, -0.1)),
            &Vector::new(0.0, 1.0, 0.0)
        );
        assert_approx_eq!(
            c.local_normal_at(&Point::new(0.3, -1.0, -0.7)),
            &Vector::new(0.0, -1.0, 0.0)
        );
        assert_approx_eq!(
            c.local_normal_at(&Point::new(-0.6, 0.3, 1.0)),
            &Vector::new(0.0, 0.0, 1.0)
        );
        assert_approx_eq!(
            c.local_normal_at(&Point::new(0.4, 0.4, -1.0)),
            &Vector::new(0.0, 0.0, -1.0)
        );
        assert_approx_eq!(
            c.local_normal_at(&Point::new(1.0, 1.0, 1.0)),
            &Vector::new(1.0, 0.0, 0.0)
        );
        assert_approx_eq!(
            c.local_normal_at(&Point::new(-1.0, -1.0, -1.0)),
            &Vector::new(-1.0, 0.0, 0.0)
        );
    }
}
