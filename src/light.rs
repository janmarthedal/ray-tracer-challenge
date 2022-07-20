use crate::color::{Color, BLACK};
use crate::material::Material;
use crate::point::Point;
use crate::vector::{reflect, Vector};

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
}

pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: &Point,
    eyev: &Vector,
    normalv: &Vector,
) -> Color {
    // combine the surface color with the light's color/intensity
    let effective_color = material.color * light.intensity;
    // find the direction to the light source
    let lightv = (light.position - point).normalize(); // compute the ambient contribution
    let ambient = effective_color * material.ambient;
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
        diffuse = effective_color * material.diffuse * light_dot_normal;
        // reflect_dot_eye represents the cosine of the angle between the
        // reflection vector and the eye vector. A negative number means the
        // light reflects away from the eye.
        let reflectv = reflect(&-lightv, normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        if reflect_dot_eye <= 0.0 {
            specular = BLACK;
        } else {
            // compute the specular contribution
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    // Add the three contributions together to get the final shading
    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::material::Material;
    use crate::point::ORIGIN;

    #[test]
    fn test_a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        assert_approx_eq!(light.position, position);
        assert_approx_eq!(light.intensity, intensity);
    }

    #[test]
    fn test_lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = ORIGIN;
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_approx_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_lighting_with_the_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::new();
        let position = ORIGIN;
        let eyev = Vector::new(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_approx_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_lighting_with_the_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = ORIGIN;
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_approx_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn test_lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = ORIGIN;
        let eyev = Vector::new(0.0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_approx_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn test_lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = ORIGIN;
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_approx_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
