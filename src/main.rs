mod approx_eq;
mod camera;
mod canvas;
mod color;
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
use light::PointLight;
use material::Material;
use pattern::{CheckersPattern, StripedPattern};
use plane::Plane;
use point::Point;
use shape::Shape;
use sphere::Sphere;
use std::f64::consts::PI;
use std::fs;
use transform::{rotation_y, scaling, translation, view_transform, IDENTITY_AFFINE};
use vector::Vector;
use world::World;

fn main() {
    let mut world = World::new();
    world.add_light(PointLight::new(Point::new(-10.0, 10.0, -10.0), WHITE));
    world.add_light(PointLight::new(
        Point::new(20.0, 10.0, -10.0),
        Color::new(0.5, 0.5, 0.5),
    ));

    let wall_material = Material::new()
        .set_pattern(
            CheckersPattern::new(Color::new(1.0, 0.9, 0.9), Color::new(0.5, 0.45, 0.45)),
            IDENTITY_AFFINE,
        )
        .set_specular(0.0)
        .set_reflective(0.5);

    world.add_shape(Shape::new(Plane::new()).set_material(wall_material));

    // middle sphere
    world.add_shape(
        Shape::new(Sphere::new())
            .set_transform(translation(-0.5, 1.0, 0.5) * &rotation_y(PI / 4.0))
            .set_material(
                Material::new()
                    .set_pattern(
                        StripedPattern::new(Color::new(0.1, 1.0, 0.5), Color::new(0.1, 0.5, 1.0)),
                        scaling(0.1, 0.1, 0.1),
                    )
                    .set_diffuse(0.7)
                    .set_specular(0.3),
            ),
    );

    // right sphere
    world.add_shape(
        Shape::new(Sphere::new())
            .set_transform(translation(1.5, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5))
            .set_material(
                Material::new()
                    .set_color(Color::new(0.5, 1.0, 0.1))
                    .set_diffuse(0.7)
                    .set_specular(0.3)
                    .set_reflective(0.5),
            ),
    );

    // left sphere
    world.add_shape(
        Shape::new(Sphere::new())
            .set_transform(translation(-1.5, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33))
            .set_material(
                Material::new()
                    .set_color(Color::new(1.0, 0.8, 0.1))
                    .set_diffuse(0.7)
                    .set_specular(0.3),
            ),
    );

    let camera = Camera::new(1600, 800, PI / 3.0).set_transform(view_transform(
        &Point::new(0.0, 1.5, -6.0),
        &Point::new(0.0, 1.0, -1.0),
        &Vector::new(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);

    fs::write("canvas.ppm", canvas.to_ppm()).expect("Unable to write file");
}
