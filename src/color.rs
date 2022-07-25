use crate::approx_eq::ApproxEq;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};
pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }
}

impl ApproxEq for Color {
    fn approx_eq(&self, other: &Self) -> bool {
        self.red.approx_eq(&other.red)
            && self.green.approx_eq(&other.green)
            && self.blue.approx_eq(&other.blue)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Self::Output {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self::Output {
        Color {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self::Output {
        &self * other
    }
}

/* impl Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, other: &Color) -> Self::Output {
        Color {
            red: self * other.red,
            green: self * other.green,
            blue: self * other.blue,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        self * &other
    }
} */

impl Mul<&Color> for Color {
    type Output = Color;

    fn mul(self, other: &Color) -> Self::Output {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::assert_approx_eq;

    #[test]
    fn test_colors_are_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_approx_eq!(c.red, -0.5);
        assert_approx_eq!(c.green, 0.4);
        assert_approx_eq!(c.blue, 1.7);
    }

    #[test]
    fn test_adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_approx_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_approx_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_approx_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_approx_eq!(c1 * &c2, Color::new(0.9, 0.2, 0.04));
    }
}
