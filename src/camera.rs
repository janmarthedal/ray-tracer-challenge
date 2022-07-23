use crate::canvas::Canvas;
use crate::point::{Point, ORIGIN};
use crate::ray::Ray;
use crate::transform::{Affine, IDENTITY_AFFINE};
use crate::world::World;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    transform: Affine,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let half_width: f64;
        let half_height: f64;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.0) / hsize as f64;
        Self {
            hsize,
            vsize,
            half_width,
            half_height,
            pixel_size,
            transform: IDENTITY_AFFINE,
        }
    }
    pub fn set_transform(&self, transform: Affine) -> Self {
        Self { transform, ..*self }
    }
    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;
        // the untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1)
        let inv_transform = self.transform.inverse().unwrap();
        let pixel = inv_transform * &Point::new(world_x, world_y, -1.0);
        let origin = inv_transform * &ORIGIN;
        let direction = (pixel - &origin).normalize();

        return Ray::new(origin, direction);
    }
    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(x, y, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::transform::{rotation_y, translation};
    use crate::vector::Vector;
    use std::f64::consts::PI;

    #[test]
    fn test_the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert_approx_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn test_the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert_approx_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn test_constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_approx_eq!(r.origin, ORIGIN);
        assert_approx_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_approx_eq!(r.origin, ORIGIN);
        assert_approx_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn test_constructing_a_ray_when_the_camera_is_transformed() {
        let c = Camera::new(201, 101, PI / 2.0)
            .set_transform(rotation_y(PI / 4.0) * &translation(0.0, -2.0, 5.0));
        let r = c.ray_for_pixel(100, 50);
        assert_approx_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_approx_eq!(
            r.direction,
            Vector::new(2f64.sqrt() / 2.0, 0.0, -2f64.sqrt() / 2.0)
        );
    }
}
