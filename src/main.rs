mod approx_eq;
mod canvas;
mod color;
mod matrix;
mod ray;
mod sphere;
mod transform;
mod tuple;

use std::f64::consts::PI;
use std::fs;
use canvas::Canvas;
use color::Color;
use transform::{rotation_y, scaling, translation};
use tuple::new_point;

fn main() {
    const SIZE: usize = 400;

    let mut c = Canvas::new(SIZE, SIZE);
    let fg = Color::new(0.9, 0.9, 0.9);
    let twelve = new_point(0.0, 0.0, 1.0);
    let scale_coef = 3.0 / 8.0 * SIZE as f64;
    let scale = scaling(scale_coef, scale_coef, scale_coef);
    let translate = translation(SIZE as f64 / 2.0, 0.0, SIZE as f64 / 2.0);

    for hour in 0..12 {
        let rotation = rotation_y(2.0 * PI * hour as f64 / 12.0);
        let p = translate * &scale * &rotation * &twelve;
        c.write_pixel(p.x.round() as usize, p.z.round() as usize, fg);
    }

    fs::write("canvas.ppm", c.to_ppm()).expect("Unable to write file");
}
