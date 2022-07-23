mod approx_eq;
mod camera;
mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod point;
mod ray;
mod sphere;
mod transform;
mod vector;
mod world;

use camera::Camera;
use color::{Color, WHITE};
use light::PointLight;
use material::Material;
use point::Point;
use sphere::Sphere;
use std::f64::consts::PI;
use std::fs;
use transform::{rotation_x, rotation_y, scaling, translation, view_transform};
use vector::Vector;
use world::World;

fn main() {
    let mut world = World::new();
    world.add_light(PointLight::new(Point::new(-10.0, 10.0, -10.0), WHITE));

    let wall_material = Material::new()
        .set_color(Color::new(1.0, 0.9, 0.9))
        .set_specular(0.0);

    // floor
    world.add_shape(
        Sphere::new()
            .set_transform(scaling(10.0, 0.01, 10.0))
            .set_material(wall_material),
    );

    // left wall
    world.add_shape(
        Sphere::new()
            .set_transform(
                translation(0.0, 0.0, 5.0)
                    * &rotation_y(-PI / 4.0)
                    * &rotation_x(PI / 2.0)
                    * &scaling(10.0, 0.01, 10.0),
            )
            .set_material(wall_material),
    );

    // right wall
    world.add_shape(
        Sphere::new()
            .set_transform(
                translation(0.0, 0.0, 5.0)
                    * &rotation_y(PI / 4.0)
                    * &rotation_x(PI / 2.0)
                    * &scaling(10.0, 0.01, 10.0),
            )
            .set_material(wall_material),
    );

    // middle sphere
    world.add_shape(
        Sphere::new()
            .set_transform(translation(-0.5, 1.0, 0.5))
            .set_material(
                Material::new()
                    .set_color(Color::new(0.1, 1.0, 0.5))
                    .set_diffuse(0.7)
                    .set_specular(0.3),
            ),
    );

    // right sphere
    world.add_shape(
        Sphere::new()
            .set_transform(translation(1.5, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5))
            .set_material(
                Material::new()
                    .set_color(Color::new(0.5, 1.0, 0.1))
                    .set_diffuse(0.7)
                    .set_specular(0.3),
            ),
    );

    // left sphere
    world.add_shape(
        Sphere::new()
            .set_transform(translation(-1.5, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33))
            .set_material(
                Material::new()
                    .set_color(Color::new(1.0, 0.8, 0.1))
                    .set_diffuse(0.7)
                    .set_specular(0.3),
            ),
    );

    let camera = Camera::new(800, 400, PI / 3.0).set_transform(view_transform(
        &Point::new(0.0, 1.5, -5.0),
        &Point::new(0.0, 1.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);

    fs::write("canvas.ppm", canvas.to_ppm()).expect("Unable to write file");
}
