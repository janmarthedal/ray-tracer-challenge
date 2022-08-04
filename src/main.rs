mod approx_eq;
mod camera;
mod canvas;
mod color;
mod cube;
mod cylinder;
mod intersection;
mod light;
mod material;
mod matrix;
mod pattern;
mod plane;
mod point;
mod ray;
mod shape;
mod sphere;
mod transform;
mod vector;
mod world;

use camera::Camera;
use color::{Color, WHITE};
use cube::Cube;
use light::PointLight;
use material::Material;
use pattern::CheckersPattern;
use plane::Plane;
use point::Point;
use shape::Shape;
use std::f64::consts::PI;
use std::fs;
use transform::{translation, view_transform, IDENTITY_AFFINE};
use vector::Vector;
use world::World;

fn main() {
    let mut world = World::new();
    world.add_light(PointLight::new(Point::new(-10.0, 10.0, -10.0), WHITE));

    // floor
    world.add_shape(
        Shape::new(Plane::new()).set_material(
            Material::new()
                .set_pattern(
                    CheckersPattern::new(Color::new(1.0, 0.9, 0.9), Color::new(0.5, 0.45, 0.45)),
                    IDENTITY_AFFINE,
                )
                .set_specular(0.0),
        ),
    );

    // cube
    world.add_shape(
        Shape::new(Cube::new())
            .set_transform(translation(0.0, 1.0, 0.5))
            .set_material(
                Material::new()
                    .set_color(Color::new(0.1, 1.0, 0.5))
                    .set_diffuse(0.7)
                    .set_specular(0.0)
                    .set_transparency(1.0)
                    .set_refractive_index(1.5)
                    .set_reflective(0.9),
            ),
    );

    let camera = Camera::new(800, 400, PI / 3.0).set_transform(view_transform(
        &Point::new(2.0, 4.0, -6.0),
        &Point::new(0.0, 1.0, -1.0),
        &Vector::new(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);

    fs::write("canvas.ppm", canvas.to_ppm()).expect("Unable to write file");
}
