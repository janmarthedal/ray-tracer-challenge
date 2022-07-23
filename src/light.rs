use crate::color::Color;
use crate::point::Point;
use crate::vector::Vector;

pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
    pub fn combine(&self, color: &Color) -> Color {
        self.intensity * color
    }
    pub fn scale_intensity(&self, factor: f64) -> Color {
        self.intensity * factor
    }
    pub fn vector_from(&self, point: &Point) -> Vector {
        self.position - point
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};

    #[test]
    fn test_a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        assert_approx_eq!(light.position, position);
        assert_approx_eq!(light.intensity, intensity);
    }
}
