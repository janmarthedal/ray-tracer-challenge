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
    pub fn new<IntoIter: IntoIterator<Item=Intersection>>(collection: IntoIter) -> Self {
        let mut intersections = collection.into_iter().collect::<Vec<_>>();
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Self {
            intersections
        }
    }
    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections.iter().find(|i| i.t >= 0.0)
    }
    #[cfg(test)]
    pub fn get(&self) -> Vec<f64> {
        self.intersections.iter().map(|i| i.t).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_an_intersection_encapsulates_t_and_object() {
        let i = Intersection::new(3.5, 1);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object_id, 1);
    }

    #[test]
    fn test_aggregating_intersections() {
        let i1 = Intersection::new(1.0, 1);
        let i2 = Intersection::new(2.0, 1);
        let xs = Intersections::new([i1, i2]);
        assert_eq!(xs.intersections.len(), 2);
        assert_eq!(xs.intersections[0].t, 1.0);
        assert_eq!(xs.intersections[1].t, 2.0);
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_positive_t() {
        let i1 = Intersection::new(1.0, 1);
        let i2 = Intersection::new(2.0, 1);
        let expect = i1.clone();
        let xs = Intersections::new([i1, i2]);
        let i = xs.hit();
        assert_eq!(i, Some(&expect));
    }

    #[test]
    fn test_the_hit_when_some_intersections_have_negative_t() {
        let i1 = Intersection::new(-1.0, 1);
        let i2 = Intersection::new(1.0, 1);
        let expect = i2.clone();
        let xs = Intersections::new([i2, i1]);
        let i = xs.hit();
        assert_eq!(i, Some(&expect));
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_negative_t() {
        let i1 = Intersection::new(-2.0, 1);
        let i2 = Intersection::new(-1.0, 1);
        let xs = Intersections::new([i2, i1]);
        let i = xs.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn test_the_hit_is_always_the_lowest_nonnegative_intersection() {
        let i1 = Intersection::new(5.0, 1);
        let i2 = Intersection::new(7.0, 1);
        let i3 = Intersection::new(-3.0, 1);
        let i4 = Intersection::new(2.0, 1);
        let expect = i4.clone();
        let xs = Intersections::new([i1, i2, i3, i4]);
        let i = xs.hit();
        assert_eq!(i, Some(&expect));
    }
}
