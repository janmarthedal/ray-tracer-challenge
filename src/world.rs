use crate::approx_eq::EPSILON;
use crate::color::{Color, BLACK};
use crate::intersection::{Intersection, Intersections};
use crate::light::PointLight;
use crate::local_shape::LocalShape;
use crate::material::{Material, DEFAULT_MATERIAL};
use crate::point::Point;
use crate::ray::Ray;
use crate::transform::{Affine, IDENTITY_AFFINE};
use crate::vector::Vector;

pub struct Shape<'a> {
    transform: Affine,
    material: Material,
    local_shape: Box<dyn LocalShape + 'a>,
}

impl<'a> Shape<'a> {
    pub fn new(local_shape: impl LocalShape + 'a) -> Self {
        Self {
            transform: IDENTITY_AFFINE,
            material: DEFAULT_MATERIAL,
            local_shape: Box::new(local_shape),
        }
    }
    pub fn set_transform(self, transform: Affine) -> Self {
        Self { transform, ..self }
    }
    pub fn set_material(self, material: Material) -> Self {
        Self { material, ..self }
    }
    fn get_material(&self) -> &Material {
        &self.material
    }
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let ray = ray.transform(&self.transform.inverse().unwrap());
        self.local_shape.local_intersect(&ray)
    }
    fn normal_at(&self, point: &Point) -> Vector {
        let inverse_transform = self.transform.inverse().unwrap();
        let local_point = inverse_transform * point;
        let local_normal = self.local_shape.local_normal_at(&local_point);
        let world_normal = inverse_transform.get_transform().transpose() * &local_normal;
        world_normal.normalize()
    }
}

pub struct World<'a> {
    lights: Vec<PointLight>,
    objects: Vec<Shape<'a>>,
    handle_shadows: bool,
}

struct Computations {
    t: f64,
    object_id: usize,
    point: Point,
    over_point: Point,
    eyev: Vector,
    normalv: Vector,
    inside: bool,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        Self {
            lights: vec![],
            objects: vec![],
            handle_shadows: true,
        }
    }
    #[cfg(test)]
    fn new_no_shadows() -> Self {
        Self {
            lights: vec![],
            objects: vec![],
            handle_shadows: false,
        }
    }
    #[cfg(test)]
    pub fn clear_lights(&mut self) {
        self.lights.clear();
    }
    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }
    pub fn add_shape(&mut self, object: Shape<'a>) -> usize {
        let id = self.objects.len();
        self.objects.push(object);
        id
    }
    fn intersect(&self, ray: &Ray) -> Intersections {
        let mut xs = Intersections::new();
        for (i, obj) in self.objects.iter().enumerate() {
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
        let nv = self.objects[intersection.object_id].normal_at(&point);
        let inside = nv.dot(&eyev) < 0.0;
        let normalv = if inside { -nv } else { nv };
        let over_point = point + &(&normalv * EPSILON);
        Computations {
            t: intersection.t,
            object_id: intersection.object_id,
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }
    fn shade_hit(&self, comps: &Computations) -> Color {
        let mut result = BLACK;
        let material = self.objects[comps.object_id].get_material();
        for light in &self.lights {
            let shadowed = self.handle_shadows && self.is_shadowed(light, &comps.over_point);
            let color = material.lighting(
                &light,
                &comps.over_point,
                &comps.eyev,
                &comps.normalv,
                shadowed,
            );
            result = result + color;
        }
        result
    }
    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(ray);
        if let Some(intersection) = intersections.hit() {
            let comps = self.prepare_computations(intersection, ray);
            self.shade_hit(&comps)
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
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::color::WHITE;
    use crate::material::Material;
    use crate::sphere::Sphere;
    use crate::transform::{rotation_z, scaling, translation};

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
    fn test_a_shapes_default_transformation() {
        let s = Shape::new(Sphere::new());
        assert_approx_eq!(s.transform, &IDENTITY_AFFINE);
    }

    #[test]
    fn test_changing_a_shapes_transformation() {
        let t = translation(2.0, 3.0, 4.0);
        let s = Shape::new(Sphere::new()).set_transform(t);
        assert_approx_eq!(s.transform, &t);
    }

    #[test]
    fn test_intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::new(Sphere::new()).set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, [3.0, 7.0]);
    }

    #[test]
    fn test_intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::new(Sphere::new()).set_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_approx_eq!(xs, []);
    }

    #[test]
    fn test_computing_the_normal_on_a_translated_sphere() {
        let s = Shape::new(Sphere::new()).set_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&Point::new(0.0, 1.70711, -0.70711));
        assert_approx_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn test_computing_the_normal_on_a_transformed_sphere() {
        let s = Shape::new(Sphere::new())
            .set_transform(scaling(1.0, 0.5, 1.0) * &rotation_z(std::f64::consts::PI / 5.0));
        let n = s.normal_at(&Point::new(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0));
        assert_approx_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
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
        let c = w.shade_hit(&comps);
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
        let c = w.shade_hit(&comps);
        assert_approx_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_approx_eq!(c, BLACK);
    }

    #[test]
    fn test_the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
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
        let c = world.color_at(&r);
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
}
