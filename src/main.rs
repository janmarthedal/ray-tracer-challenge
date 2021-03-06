mod approx_eq;
mod canvas;
mod color;
mod intersection;
mod matrix;
mod ray;
mod sphere;
mod transform;
mod tuple;
mod world;

use std::fs;
use canvas::Canvas;
use color::Color;
use intersection::{Intersection, Intersections};
use ray::Ray;
use sphere::Sphere;
use transform::{scaling, shearing};
use tuple::new_point;
use world::Object;

fn intersect_object(shape: &dyn Object, ray: &Ray) -> Intersections {
    let mut intersections = Intersections::new();
    let xs = shape.intersect(ray);
    for t in xs {
        intersections.add(Intersection::new(t, shape.get_id()));
    }
    intersections
}

fn main() {
    let ray_origin = new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);

    let mut shape = Sphere::new(0);
    shape.set_transform(shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * &scaling(0.5, 1.0, 1.0));

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = new_point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let intersections = intersect_object(&shape, &r);

            if intersections.hit().is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    fs::write("canvas.ppm", canvas.to_ppm()).expect("Unable to write file");
}
