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
    object_id: usize,
    over_point: Point,
    under_point: Point,
    eyev: Vector,
    normalv: Vector,
    reflectv: Vector,
    n1: f64,
    n2: f64,
    #[cfg(test)]
    t: f64,
    #[cfg(test)]
    point: Point,
    #[cfg(test)]
    inside: bool,
}

impl Computations {
    fn schlick(&self) -> f64 {
        // find the cosine of the angle between the eye and normal vectors
        let mut cos = self.eyev.dot(&self.normalv);
        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n * n * (1.0 - cos * cos);
            if sin2_t > 1.0 {
                return 1.0;
            }
            // compute cosine of theta_t using trig identity
            let cos_t = (1.0 - sin2_t).sqrt();
            // when n1 > n2, use cos(theta_t) instead
            cos = cos_t
        }
        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
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
        Intersections::new(self.shapes.iter().enumerate().flat_map(|(i, obj)| {
            obj.intersect(ray)
                .iter()
                .map(|t| Intersection::new(*t, i))
                .collect::<Vec<_>>()
        }))
    }
    fn prepare_computations(
        &self,
        intersections: Intersections,
        intersection_index: usize,
        ray: &Ray,
    ) -> Computations {
        let intersections: Vec<Intersection> = Vec::from(intersections);
        let intersection = intersections[intersection_index];
        let point = ray.position(intersection.t);
        let eyev = -ray.direction;
        let nv = self.shapes[intersection.object_id].normal_at(&point);
        let inside = nv.dot(&eyev) < 0.0;
        let normalv = if inside { -nv } else { nv };
        let reflectv = reflect(&ray.direction, &normalv);
        let over_point = point + &(&normalv * EPSILON);
        let under_point = point - &(&normalv * EPSILON);
        let mut containers: Vec<usize> = vec![];
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        for (index, i) in intersections.iter().enumerate() {
            if index == intersection_index {
                if let Some(object_id) = containers.last() {
                    n1 = self.shapes[*object_id]
                        .get_material()
                        .get_refractive_index();
                }
            }
            match containers.iter().position(|c| *c == i.object_id) {
                Some(p) => {
                    containers.remove(p);
                }
                None => containers.push(i.object_id),
            };
            if index == intersection_index {
                if let Some(object_id) = containers.last() {
                    n2 = self.shapes[*object_id]
                        .get_material()
                        .get_refractive_index();
                }
                break;
            }
        }
        Computations {
            object_id: intersection.object_id,
            over_point,
            under_point,
            eyev,
            normalv,
            reflectv,
            n1,
            n2,
            #[cfg(test)]
            t: intersection.t,
            #[cfg(test)]
            point,
            #[cfg(test)]
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
        let refracted = self.refracted_color(comps, remaining);

        if material.is_reflective() && material.is_transparent() {
            let reflectance = comps.schlick();
            return surface + reflected * reflectance + refracted * (1.0 - reflectance);
        }

        surface + reflected + refracted
    }
    pub fn color_at(&self, ray: &Ray, remaining: isize) -> Color {
        let intersections = self.intersect(ray);
        if let Some(intersection_index) = intersections.hit_index() {
            let comps = self.prepare_computations(intersections, intersection_index, ray);
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

        if let Some(intersection_index) = intersections.hit_index() {
            let intersection = Vec::from(intersections)[intersection_index];
            if intersection.t < distance {
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
    fn refracted_color(&self, comps: &Computations, remaining: isize) -> Color {
        let material = self.shapes[comps.object_id].get_material();
        if !material.is_transparent() || remaining <= 0 {
            return BLACK;
        }
        // Find the ratio of first index of refraction to the second.
        // (Yup, this is inverted from the definition of Snell's Law.)
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot(&comps.normalv);
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);
        if sin2_t > 1.0 {
            // total internal reflection
            return BLACK;
        }
        let cos_t = (1.0 - sin2_t).sqrt();
        // Compute the direction of the refracted ray
        let direction = (n_ratio * cos_i - cos_t) * &comps.normalv - &(n_ratio * &comps.eyev);
        // Create the refracted ray
        let refract_ray = Ray::new(comps.under_point, direction);
        // Find the color of the refracted ray, making sure to multiply
        // by the transparency value to account for any opacity
        material.scale_transparency(&self.color_at(&refract_ray, remaining - 1))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::color::WHITE;
    use crate::material::Material;
    use crate::pattern::Pattern;
    use crate::plane::Plane;
    use crate::point::ORIGIN;
    use crate::sphere::Sphere;
    use crate::transform::{scaling, translation, Affine, IDENTITY_AFFINE};

    impl<'a> World<'a> {
        fn clear_lights(&mut self) {
            self.lights.clear();
        }
    }

    fn default_light() -> PointLight {
        PointLight::new(Point::new(-10.0, 10.0, -10.0), WHITE)
    }

    fn default_world<'a>() -> World<'a> {
        let mut world = World::new();
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

    fn new_glass_sphere<'a>(transform: Affine, refractive_index: f64) -> Shape<'a> {
        Shape::new(Sphere::new())
            .set_material(
                Material::new()
                    .set_transparency(1.0)
                    .set_refractive_index(refractive_index),
            )
            .set_transform(transform)
    }

    struct TestPattern {}

    impl TestPattern {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Pattern for TestPattern {
        fn get_color(&self, point: &Point) -> Color {
            Color::new(point.x, point.y, point.z)
        }
    }

    #[test]
    fn test_intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);
        assert_approx_eq!(
            Vec::from(xs).iter().map(|i| i.t).collect::<Vec<_>>(),
            [4.0, 4.5, 5.5, 6.0]
        );
    }

    #[test]
    fn test_precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut world = World::new();
        let shape = world.add_shape(Shape::new(Sphere::new()));
        let i = Intersection::new(4.0, shape);
        let comp = world.prepare_computations(Intersections::new([i]), 0, &r);
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
        let comp = world.prepare_computations(Intersections::new([i]), 0, &r);
        assert!(!comp.inside);
    }

    #[test]
    fn test_the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut world = World::new();
        let shape = world.add_shape(Shape::new(Sphere::new()));
        let i = Intersection::new(1.0, shape);
        let comp = world.prepare_computations(Intersections::new([i]), 0, &r);
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
        let comps = w.prepare_computations(Intersections::new([i]), 0, &r);
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
        let comps = w.prepare_computations(Intersections::new([i]), 0, &r);
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
        let comps = w.prepare_computations(Intersections::new([i]), 0, &r);
        assert_approx_eq!(
            &comps.reflectv,
            Vector::new(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0)
        )
    }

    #[test]
    fn test_the_reflected_color_for_a_nonreflective_material() {
        let mut world = World::new();
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
        let comps = world.prepare_computations(Intersections::new([i]), 0, &r);
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
        let comps = w.prepare_computations(Intersections::new([i]), 0, &r);
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
        let comps = w.prepare_computations(Intersections::new([i]), 0, &r);
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
        let comps = w.prepare_computations(Intersections::new([i]), 0, &r);
        let color = w.reflected_color(&comps, 0);
        assert_approx_eq!(color, BLACK);
    }

    #[test]
    fn test_finding_n1_and_n2_at_various_intersections() {
        let mut world = World::new();
        let a = world.add_shape(new_glass_sphere(scaling(2.0, 2.0, 2.0), 1.5));
        let b = world.add_shape(new_glass_sphere(translation(0.0, 0.0, -0.25), 2.0));
        let c = world.add_shape(new_glass_sphere(translation(0.0, 0.0, 0.25), 2.5));
        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
        let xs = Intersections::new([
            Intersection::new(2.0, a),
            Intersection::new(2.75, b),
            Intersection::new(3.25, c),
            Intersection::new(4.75, b),
            Intersection::new(5.25, c),
            Intersection::new(6.0, a),
        ]);
        let comps = world.prepare_computations(xs.clone(), 0, &r);
        assert_approx_eq!(comps.n1, 1.0);
        assert_approx_eq!(comps.n2, 1.5);
        let comps = world.prepare_computations(xs.clone(), 1, &r);
        assert_approx_eq!(comps.n1, 1.5);
        assert_approx_eq!(comps.n2, 2.0);
        let comps = world.prepare_computations(xs.clone(), 2, &r);
        assert_approx_eq!(comps.n1, 2.0);
        assert_approx_eq!(comps.n2, 2.5);
        let comps = world.prepare_computations(xs.clone(), 3, &r);
        assert_approx_eq!(comps.n1, 2.5);
        assert_approx_eq!(comps.n2, 2.5);
        let comps = world.prepare_computations(xs.clone(), 4, &r);
        assert_approx_eq!(comps.n1, 2.5);
        assert_approx_eq!(comps.n2, 1.5);
        let comps = world.prepare_computations(xs.clone(), 5, &r);
        assert_approx_eq!(comps.n1, 1.5);
        assert_approx_eq!(comps.n2, 1.0);
    }

    #[test]
    fn test_the_refracted_color_with_an_opaque_surface() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, -1.0));
        let xs = Intersections::new([Intersection::new(4.0, 0), Intersection::new(6.0, 0)]);
        let comps = w.prepare_computations(xs, 0, &r);
        let c = w.refracted_color(&comps, 5);
        assert_approx_eq!(c, BLACK);
    }

    #[test]
    fn test_the_refracted_color_under_total_internal_reflection() {
        let mut world = World::new();
        world.add_light(default_light());
        let o1 = world.add_shape(
            Shape::new(Sphere::new()).set_material(
                Material::new()
                    .set_color(Color::new(0.8, 1.0, 0.6))
                    .set_diffuse(0.7)
                    .set_specular(0.2)
                    .set_transparency(1.0)
                    .set_refractive_index(1.5),
            ),
        );
        world.add_shape(Shape::new(Sphere::new()).set_transform(scaling(0.5, 0.5, 0.5)));
        let r = Ray::new(
            Point::new(0.0, 0.0, 2f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let xs = Intersections::new([
            Intersection::new(-2f64.sqrt() / 2.0, o1),
            Intersection::new(2f64.sqrt() / 2.0, o1),
        ]);
        let comps = world.prepare_computations(xs, 1, &r);
        let color = world.refracted_color(&comps, 5);
        assert_approx_eq!(color, BLACK);
    }

    #[test]
    fn test_the_refracted_color_with_a_refracted_ray() {
        let mut world = World::new();
        world.add_light(default_light());
        let a = world.add_shape(
            Shape::new(Sphere::new()).set_material(
                Material::new()
                    .set_pattern(TestPattern::new(), IDENTITY_AFFINE)
                    .set_diffuse(0.7)
                    .set_specular(0.2)
                    .set_ambient(1.0),
            ),
        );
        let b = world.add_shape(
            Shape::new(Sphere::new())
                .set_transform(scaling(0.5, 0.5, 0.5))
                .set_material(
                    Material::new()
                        .set_transparency(1.0)
                        .set_refractive_index(1.5),
                ),
        );
        let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
        let xs = Intersections::new([
            Intersection::new(-0.9899, a),
            Intersection::new(-0.4899, b),
            Intersection::new(0.4899, b),
            Intersection::new(0.9899, a),
        ]);
        let comps = world.prepare_computations(xs, 2, &r);
        let color = world.refracted_color(&comps, 5);
        assert_approx_eq!(color, Color::new(0.0, 0.99887, 0.04722));
    }

    #[test]
    fn test_shade_hit_with_a_transparent_material() {
        let mut w = World::new();
        w.add_light(default_light());
        w.add_shape(
            Shape::new(Sphere::new()).set_material(
                Material::new()
                    .set_color(Color::new(0.8, 1.0, 0.6))
                    .set_diffuse(0.7)
                    .set_specular(0.2),
            ),
        );
        w.add_shape(Shape::new(Sphere::new()).set_transform(scaling(0.5, 0.5, 0.5)));

        let floor = w.add_shape(
            Shape::new(Plane::new())
                .set_transform(translation(0.0, -1.0, 0.0))
                .set_material(
                    Material::new()
                        .set_transparency(0.5)
                        .set_refractive_index(1.5),
                ),
        );
        w.add_shape(
            Shape::new(Sphere::new())
                .set_transform(translation(0.0, -3.5, -0.5))
                .set_material(
                    Material::new()
                        .set_color(Color::new(1.0, 0.0, 0.0))
                        .set_ambient(0.5),
                ),
        );
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
        );
        let xs = Intersections::new([Intersection::new(2f64.sqrt(), floor)]);
        let comps = w.prepare_computations(xs, 0, &r);
        let color = w.shade_hit(&comps, 5);
        assert_approx_eq!(color, Color::new(0.93642, 0.68642, 0.68642))
    }

    #[test]
    fn test_the_schlick_approximation_under_total_internal_reflection() {
        let mut w = World::new();
        let shape = w.add_shape(new_glass_sphere(IDENTITY_AFFINE, 1.5));
        let r = Ray::new(
            Point::new(0.0, 0.0, 2f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let xs = Intersections::new([
            Intersection::new(-2f64.sqrt() / 2.0, shape),
            Intersection::new(2f64.sqrt() / 2.0, shape),
        ]);
        let comps = w.prepare_computations(xs, 1, &r);
        let reflectance = comps.schlick();
        assert_approx_eq!(reflectance, 1.0);
    }

    #[test]
    fn test_the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let mut w = World::new();
        let shape = w.add_shape(new_glass_sphere(IDENTITY_AFFINE, 1.5));
        let r = Ray::new(ORIGIN, Vector::new(0.0, 1.0, 0.0));
        let xs = Intersections::new([
            Intersection::new(-1.0, shape),
            Intersection::new(1.0, shape),
        ]);
        let comps = w.prepare_computations(xs, 1, &r);
        let reflectance = comps.schlick();
        assert_approx_eq!(reflectance, 0.04);
    }

    #[test]
    fn test_the_schlick_approximation_with_small_angle_and_n2_over_n1() {
        let mut w = World::new();
        let shape = w.add_shape(new_glass_sphere(IDENTITY_AFFINE, 1.5));
        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs = Intersections::new([Intersection::new(1.8589, shape)]);
        let comps = w.prepare_computations(xs, 0, &r);
        let reflectance = comps.schlick();
        assert_approx_eq!(reflectance, 0.48873);
    }
    #[test]
    fn test_shade_hit_with_a_reflective_transparent_material() {
        let mut w = default_world();
        let floor = w.add_shape(
            Shape::new(Plane::new())
                .set_transform(translation(0.0, -1.0, 0.0))
                .set_material(
                    Material::new()
                        .set_reflective(0.5)
                        .set_transparency(0.5)
                        .set_refractive_index(1.5),
                ),
        );
        w.add_shape(
            Shape::new(Sphere::new())
                .set_transform(translation(0.0, -3.5, -0.5))
                .set_material(
                    Material::new()
                        .set_color(Color::new(1.0, 0.0, 0.0))
                        .set_ambient(0.5),
                ),
        );
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
        );
        let xs = Intersections::new([Intersection::new(2f64.sqrt(), floor)]);
        let comps = w.prepare_computations(xs, 0, &r);
        let color = w.shade_hit(&comps, 5);
        assert_approx_eq!(color, Color::new(0.93391, 0.69643, 0.69243))
    }
}
