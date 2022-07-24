use crate::{color::Color, point::Point};

pub trait Pattern {
    fn get_color(&self, point: &Point) -> Color;
}

// StripedPattern

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

// RingPattern

pub struct RingPattern {
    c1: Color,
    c2: Color,
}

impl RingPattern {
    pub fn new(c1: Color, c2: Color) -> Self {
        Self { c1, c2 }
    }
}

impl Pattern for RingPattern {
    fn get_color(&self, point: &Point) -> Color {
        if (point.x * point.x + point.z * point.z).floor() as i32 % 2 == 0 {
            self.c1
        } else {
            self.c2
        }
    }
}

// CheckersPattern

pub struct CheckersPattern {
    c1: Color,
    c2: Color,
}

impl CheckersPattern {
    pub fn new(c1: Color, c2: Color) -> Self {
        Self { c1, c2 }
    }
}

impl Pattern for CheckersPattern {
    fn get_color(&self, point: &Point) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) as i32 % 2 == 0 {
            self.c1
        } else {
            self.c2
        }
    }
}
