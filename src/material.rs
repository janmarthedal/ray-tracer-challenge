use crate::color::{Color, BLACK, WHITE};
use crate::light::PointLight;
use crate::point::Point;
use crate::vector::{reflect, Vector};

#[derive(Clone, Copy)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

pub const DEFAULT_MATERIAL: Material = Material {
    color: WHITE,
    ambient: 0.1,
    diffuse: 0.9,
    specular: 0.9,
    shininess: 200.0,
};

impl Material {
    pub fn new() -> Self {
        Self { ..DEFAULT_MATERIAL }
    }
    pub fn set_color(&self, color: Color) -> Self {
        Self { color, ..*self }
    }
    pub fn set_ambient(&self, ambient: f64) -> Self {
        Self { ambient, ..*self }
    }
    pub fn set_diffuse(&self, diffuse: f64) -> Self {
        Self { diffuse, ..*self }
    }
    pub fn set_specular(&self, specular: f64) -> Self {
        Self { specular, ..*self }
    }
    pub fn set_shininess(&self, shininess: f64) -> Self {
        Self { shininess, ..*self }
    }
    pub fn lighting(
        &self,
        light: &PointLight,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        in_shadow: bool,
    ) -> Color {
        // combine the surface color with the light's color/intensity
        let effective_color = light.combine(&self.color);
        // compute the ambient contribution
        let ambient = effective_color * self.ambient;
        if in_shadow {
            return ambient;
        }
        // find the direction to the light source
        let lightv = light.vector_from(point).normalize();
        // light_dot_normal represents the cosine of the angle between the # light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = lightv.dot(normalv);
        let diffuse: Color;
        let specular: Color;
        if light_dot_normal < 0.0 {
            diffuse = BLACK;
            specular = BLACK;
        } else {
            // compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;
            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = reflect(&-lightv, normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye <= 0.0 {
                specular = BLACK;
            } else {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.scale_intensity(self.specular * factor);
            }
        }
        // Add the three contributions together to get the final shading
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::material::Material;
    use crate::point::ORIGIN;
    use crate::vector::Vector;

    #[test]
    fn test_lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = ORIGIN;
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_approx_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_lighting_with_the_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::new();
        let position = ORIGIN;
        let eyev = Vector::new(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_approx_eq!(result, WHITE);
    }

    #[test]
    fn test_lighting_with_the_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = ORIGIN;
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_approx_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn test_lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = ORIGIN;
        let eyev = Vector::new(0.0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_approx_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn test_lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = ORIGIN;
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_approx_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_lighting_with_the_surface_in_shadow() {
        let m = Material::new();
        let position = ORIGIN;
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), WHITE);
        let result = m.lighting(&light, &position, &eyev, &normalv, true);
        assert_approx_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
