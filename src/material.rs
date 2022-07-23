use crate::color::{Color, WHITE, BLACK};
use crate::light::PointLight;
use crate::point::Point;
use crate::vector::{Vector, reflect};

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
    ) -> Color {
        // combine the surface color with the light's color/intensity
        let effective_color = light.combine(&self.color);
        // find the direction to the light source
        let lightv = light.direction_from(point);
        // compute the ambient contribution
        let ambient = effective_color * self.ambient;
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
