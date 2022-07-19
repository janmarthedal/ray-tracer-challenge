#[derive(Debug, PartialEq, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object_id: usize,
}

impl Intersection {
    pub fn new(t: f64, object_id: usize) -> Self {
        Self { t, object_id }
    }
}

pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn new() -> Self {
        Self {
            intersections: vec![],
        }
    }
    pub fn add(&mut self, i: Intersection) {
        self.intersections.push(i);
    }
    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections
            .iter()
            .filter(|i| i.t >= 0.0)
            .fold(None, |acc, i| match acc {
                Some(best) if i.t > best.t => acc,
                _ => Some(i),
            })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::sphere::Sphere;
    use crate::world::Object;

    #[test]
    fn test_an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new(7);
        let i = Intersection::new(3.5, s.get_id());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object_id, 7);
    }

    #[test]
    fn test_aggregating_intersections() {
        let s = Sphere::new(7);
        let i1 = Intersection::new(1.0, s.get_id());
        let i2 = Intersection::new(2.0, s.get_id());
        let mut xs = Intersections::new();
        xs.add(i1);
        xs.add(i2);
        assert_eq!(xs.intersections.len(), 2);
        assert_eq!(xs.intersections[0].t, 1.0);
        assert_eq!(xs.intersections[1].t, 2.0);
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new(7);
        let i1 = Intersection::new(1.0, s.get_id());
        let i2 = Intersection::new(2.0, s.get_id());
        let mut xs = Intersections::new();
        let expect = i1.clone();
        xs.add(i1);
        xs.add(i2);
        let i = xs.hit();
        assert_eq!(i, Some(&expect));
    }

    #[test]
    fn test_the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new(7);
        let i1 = Intersection::new(-1.0, s.get_id());
        let i2 = Intersection::new(1.0, s.get_id());
        let mut xs = Intersections::new();
        let expect = i2.clone();
        xs.add(i2);
        xs.add(i1);
        let i = xs.hit();
        assert_eq!(i, Some(&expect));
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new(7);
        let i1 = Intersection::new(-2.0, s.get_id());
        let i2 = Intersection::new(-1.0, s.get_id());
        let mut xs = Intersections::new();
        xs.add(i2);
        xs.add(i1);
        let i = xs.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn test_the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new(7);
        let i1 = Intersection::new(5.0, s.get_id());
        let i2 = Intersection::new(7.0, s.get_id());
        let i3 = Intersection::new(-3.0, s.get_id());
        let i4 = Intersection::new(2.0, s.get_id());
        let mut xs = Intersections::new();
        let expect = i4.clone();
        xs.add(i1);
        xs.add(i2);
        xs.add(i3);
        xs.add(i4);
        let i = xs.hit();
        assert_eq!(i, Some(&expect));
    }
}
