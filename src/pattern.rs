use crate::{point::Point, color::Color};

pub trait Pattern {
    fn get_color(&self, point: &Point) -> Color;
}

pub struct StripedPattern {
    c1: Color,
    c2: Color,
}

impl StripedPattern {
    pub fn new(c1: Color, c2: Color) -> Self {
        Self { c1, c2 }
    }
}

impl Pattern for StripedPattern {
    fn get_color(&self, point: &Point) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.c1
        } else {
            self.c2
        }
    }
}
