use crate::approx_eq::EPSILON;
use crate::color::{Color, BLACK};
use crate::intersection::{Intersection, Intersections};
use crate::light::PointLight;
use crate::point::Point;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vector::{reflect, Vector};

pub const RECURSION_LIMIT: isize = 5;

pub struct World<'a> {
    lights: Vec<PointLight>,
    shapes: Vec<Shape<'a>>,
    handle_shadows: bool,
}

struct Computations {
    t: f64,
    object_id: usize,
    point: Point,
    over_point: Point,
    eyev: Vector,
    normalv: Vector,
    reflectv: Vector,
    inside: bool,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        Self {
            lights: vec![],
            shapes: vec![],
            handle_shadows: true,
        }
    }
    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }
    pub fn add_shape(&mut self, object: Shape<'a>) -> usize {
        let id = self.shapes.len();
        self.shapes.push(object);
        id
    }
    fn intersect(&self, ray: &Ray) -> Intersections {
        let mut xs = Intersections::new();
        for (i, obj) in self.shapes.iter().enumerate() {
            let obj_xs = obj.intersect(ray);
            for t in obj_xs {
                xs.add(Intersection::new(t, i));
            }
        }
        xs.sort();
        xs
    }
    fn prepare_computations(&self, intersection: &Intersection, ray: &Ray) -> Computations {
        let point = ray.position(intersection.t);
        let eyev = -ray.direction;
        let nv = self.shapes[intersection.object_id].normal_at(&point);
        let inside = nv.dot(&eyev) < 0.0;
        let normalv = if inside { -nv } else { nv };
        let reflectv = reflect(&ray.direction, &normalv);
        let over_point = point + &(&normalv * EPSILON);
        Computations {
            t: intersection.t,
            object_id: intersection.object_id,
            point,
            over_point,
            eyev,
            normalv,
            reflectv,
            inside,
        }
    }
    fn shade_hit(&self, comps: &Computations, remaining: isize) -> Color {
        let shape = &self.shapes[comps.object_id];
        let material = shape.get_material();

        let mut surface = BLACK;
        for light in &self.lights {
            let shadowed = self.handle_shadows && self.is_shadowed(light, &comps.over_point);
            let color = material.lighting(
                &light,
                shape.get_inverse_transform(),
                &comps.over_point,
                &comps.eyev,
                &comps.normalv,
                shadowed,
            );
            surface = surface + color;
        }
        let reflected = self.reflected_color(comps, remaining);

        surface + reflected
    }
    pub fn color_at(&self, ray: &Ray, remaining: isize) -> Color {
        let intersections = self.intersect(ray);
        if let Some(intersection) = intersections.hit() {
            let comps = self.prepare_computations(intersection, ray);
            self.shade_hit(&comps, remaining)
        } else {
            BLACK
        }
    }
    fn is_shadowed(&self, light: &PointLight, point: &Point) -> bool {
        let v = light.vector_from(point);
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(*point, direction);
        let intersections = self.intersect(&r);

        if let Some(h) = intersections.hit() {
            if h.t < distance {
                return true;
            }
        }
        false
    }
    fn reflected_color(&self, comps: &Computations, remaining: isize) -> Color {
        let material = self.shapes[comps.object_id].get_material();
        if !material.is_reflective() || remaining <= 0 {
            return BLACK;
        }
        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray, remaining - 1);

        material.reflected_color(&color)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::color::WHITE;
    use crate::material::Material;
    use crate::plane::Plane;
    use crate::point::ORIGIN;
    use crate::sphere::Sphere;
    use crate::transform::{scaling, translation};

    impl<'a> World<'a> {
        fn new_no_shadows() -> Self {
            Self {
                lights: vec![],
                shapes: vec![],
                handle_shadows: false,
            }
        }
        fn clear_lights(&mut self) {
            self.lights.clear();
        }
    }

    fn default_light() -> PointLight {
        PointLight::new(Point::new(-10.0, 10.0, -10.0), WHITE)
    }

    fn default_world<'a>() -> World<'a> {
        let mut world = World::new_no_shadows();
        world.add_light(default_light());
        world.add_shape(
            Shape::new(Sphere::new()).set_material(
                Material::new()
                    .set_color(Color::new(0.8, 1.0, 0.6))
                    .set_diffuse(0.7)
                    .set_specular(0.2),
            ),
        );
        world.add_shape(Shape::new(Sphere::new()).set_transform(scaling(0.5, 0.5, 0.5)));
        world
    }

    #[test]
    fn test_intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);
        assert_approx_eq!(
            xs.get().iter().map(|i| i.t).collect::<Vec<_>>(),
            [4.0, 4.5, 5.5, 6.0]
        );
    }

    #[test]
    fn test_precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut world = World::new();
        let shape = world.add_shape(Shape::new(Sphere::new()));
        let i = Intersection::new(4.0, shape);
        let comp = world.prepare_computations(&i, &r);
        assert_approx_eq!(comp.t, 4.0);
        assert_eq!(comp.object_id, shape);
        assert_approx_eq!(comp.point, Point::new(0.0, 0.0, -1.0));
        assert_approx_eq!(comp.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_approx_eq!(comp.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut world = World::new();
        let shape = world.add_shape(Shape::new(Sphere::new()));
        let i = Intersection::new(4.0, shape);
        let comp = world.prepare_computations(&i, &r);
        assert!(!comp.inside);
    }

    #[test]
    fn test_the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut world = World::new();
        let shape = world.add_shape(Shape::new(Sphere::new()));
        let i = Intersection::new(1.0, shape);
        let comp = world.prepare_computations(&i, &r);
        assert_approx_eq!(comp.point, Point::new(0.0, 0.0, 1.0));
        assert_approx_eq!(comp.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(comp.inside);
        assert_approx_eq!(comp.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let object_id = 0;
        let i = Intersection::new(4.0, object_id);
        let comps = w.prepare_computations(&i, &r);
        let c = w.shade_hit(&comps, RECURSION_LIMIT);
        assert_approx_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.clear_lights();
        w.add_light(PointLight::new(
            Point::new(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let object_id = 1;
        let i = Intersection::new(0.5, object_id);
        let comps = w.prepare_computations(&i, &r);
        let c = w.shade_hit(&comps, RECURSION_LIMIT);
        assert_approx_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r, RECURSION_LIMIT);
        assert_approx_eq!(c, BLACK);
    }

    #[test]
    fn test_the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r, RECURSION_LIMIT);
        assert_approx_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_the_color_with_an_intersection_behind_the_ray() {
        let mut world = World::new();
        world.add_light(PointLight::new(Point::new(-10.0, 10.0, -10.0), WHITE));
        world.add_shape(
            Shape::new(Sphere::new()).set_material(
                Material::new()
                    .set_color(Color::new(0.8, 1.0, 0.6))
                    .set_diffuse(0.7)
                    .set_specular(0.2)
                    .set_ambient(1.0),
            ),
        );
        world.add_shape(
            Shape::new(Sphere::new())
                .set_transform(scaling(0.5, 0.5, 0.5))
                .set_material(Material::new().set_ambient(1.0)),
        );

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = world.color_at(&r, RECURSION_LIMIT);
        assert_approx_eq!(c, WHITE);
    }

    #[test]
    fn test_there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&default_light(), &p));
    }

    #[test]
    fn test_the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = Point::new(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&default_light(), &p));
    }

    #[test]
    fn test_there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&default_light(), &p));
    }

    #[test]
    fn test_there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&default_light(), &p));
    }

    #[test]
    fn test_precomputing_the_reflection_vector() {
        let mut w = World::new();
        let id = w.add_shape(Shape::new(Plane::new()));
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2f64.sqrt(), id);
        let comps = w.prepare_computations(&i, &r);
        assert_approx_eq!(
            &comps.reflectv,
            Vector::new(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0)
        )
    }

    #[test]
    fn test_the_reflected_color_for_a_nonreflective_material() {
        let mut world = World::new_no_shadows();
        world.add_light(default_light());
        world.add_shape(
            Shape::new(Sphere::new()).set_material(
                Material::new()
                    .set_color(Color::new(0.8, 1.0, 0.6))
                    .set_diffuse(0.7)
                    .set_specular(0.2),
            ),
        );
        let id2 = world.add_shape(
            Shape::new(Sphere::new())
                .set_transform(scaling(0.5, 0.5, 0.5))
                .set_material(Material::new().set_ambient(1.0)),
        );
        let r = Ray::new(ORIGIN, Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(1.0, id2);
        let comps = world.prepare_computations(&i, &r);
        let color = world.reflected_color(&comps, RECURSION_LIMIT);
        assert_approx_eq!(color, BLACK);
    }

    #[test]
    fn test_the_reflected_color_for_a_reflective_material() {
        let mut w = default_world();
        let id3 = w.add_shape(
            Shape::new(Plane::new())
                .set_material(Material::new().set_reflective(0.5))
                .set_transform(translation(0.0, -1.0, 0.0)),
        );
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2f64.sqrt(), id3);
        let comps = w.prepare_computations(&i, &r);
        let color = w.reflected_color(&comps, RECURSION_LIMIT);
        assert_approx_eq!(color, Color::new(0.19033, 0.23792, 0.14275));
    }

    #[test]
    fn test_shade_hit_with_a_reflective_material() {
        let mut w = default_world();
        let id3 = w.add_shape(
            Shape::new(Plane::new())
                .set_material(Material::new().set_reflective(0.5))
                .set_transform(translation(0.0, -1.0, 0.0)),
        );
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2f64.sqrt(), id3);
        let comps = w.prepare_computations(&i, &r);
        let color = w.shade_hit(&comps, RECURSION_LIMIT);
        assert_approx_eq!(color, Color::new(0.87676, 0.92434, 0.82917));
    }

    #[test]
    fn test_color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.add_light(PointLight::new(ORIGIN, WHITE));
        w.add_shape(
            Shape::new(Plane::new())
                .set_material(Material::new().set_reflective(1.0))
                .set_transform(translation(0.0, -1.0, 0.0)),
        );
        w.add_shape(
            Shape::new(Plane::new())
                .set_material(Material::new().set_reflective(1.0))
                .set_transform(translation(0.0, 1.0, 0.0)),
        );
        let r = Ray::new(ORIGIN, Vector::new(0.0, 1.0, 0.0));
        w.color_at(&r, RECURSION_LIMIT);
    }

    #[test]
    fn test_the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();
        let id3 = w.add_shape(
            Shape::new(Plane::new())
                .set_material(Material::new().set_reflective(0.5))
                .set_transform(translation(0.0, -1.0, 0.0)),
        );
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2f64.sqrt(), id3);
        let comps = w.prepare_computations(&i, &r);
        let color = w.reflected_color(&comps, 0);
        assert_approx_eq!(color, BLACK);
    }
}
