mod approx_eq;
mod canvas;
mod color;
mod matrix;
mod transform;
mod tuple;

use tuple::Tuple;
use canvas::Canvas;
use color::Color;
use std::fs;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Self {
        Self { position, velocity }
    }
}

impl Environment {
    fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self { gravity, wind }
    }
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile::new(position, velocity)
}

fn main() {
    let start = Tuple::new_point(0.0, 1.0, 0.0);
    let velocity = Tuple::new_vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut p = Projectile::new(start, velocity);

    let gravity = Tuple::new_vector(0.0, -0.1, 0.0);
    let wind = Tuple::new_vector(-0.01, 0.0, 0.0);
    let e = Environment::new(gravity,wind);

    let mut c = Canvas::new(900, 500);
    let projectile_color = Color::new(0.8, 0.8, 0.8);

    while p.position.y > 0.0 {
        let x = p.position.x.round();
        let y = c.height() as f64 - p.position.y.round();
        if x >= 0.0 && x < c.width() as f64 && y >= 0.0 && y < c.height() as f64 {
            let x = x as usize;
            let y = y as usize;
            c.write_pixel(x, y, projectile_color);
        }
        p = tick(&e, &p);
    }

    fs::write("canvas.ppm", c.to_ppm()).expect("Unable to write file");
}
