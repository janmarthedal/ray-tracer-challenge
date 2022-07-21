use crate::color::{Color, BLACK};
use crate::intersection::{Intersection, Intersections};
use crate::light::{PointLight, lighting};
use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transform::Affine;
use crate::vector::Vector;

pub trait Object {
    fn get_id(&self) -> usize;
    fn set_transform(&mut self, transform: Affine);
    fn set_material(&mut self, material: Material);
    fn get_material(&self) -> &Material;
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
    fn normal_at(&self, p: &Point) -> Vector;
}

pub struct World {
    lights: Vec<PointLight>,
    objects: Vec<Box<dyn Object>>,
}

struct Computations {
    t: f64,
    object_id: usize,
    point: Point,
    eyev: Vector,
    normalv: Vector,
    inside: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            lights: vec![],
            objects: vec![],
        }
    }
    pub fn clear_lights(&mut self) {
        self.lights.clear();
    }
    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }
    pub fn add_sphere(&mut self) -> usize {
        let id = self.objects.len();
        let object = Sphere::new(id);
        self.objects.push(Box::new(object));
        id
    }
    pub fn set_transform(&mut self, id: usize, transform: Affine) {
        self.objects[id].set_transform(transform);
    }
    pub fn set_material(&mut self, id: usize, material: Material) {
        self.objects[id].set_material(material);
    }
    fn intersect(&self, ray: &Ray) -> Intersections {
        let mut xs = Intersections::new();
        for obj in &self.objects {
            let obj_xs = obj.intersect(ray);
            for t in obj_xs {
                xs.add(Intersection::new(t, obj.get_id()));
            }
        }
        xs.sort();
        xs
    }
    fn prepare_computations(&self, intersection: &Intersection, ray: &Ray) -> Computations {
        let point = ray.position(intersection.t);
        let eyev = -ray.direction;
        let normalv = self.objects[intersection.object_id].normal_at(&point);
        let inside = normalv.dot(&eyev) < 0.0;
        Computations {
            t: intersection.t,
            object_id: intersection.object_id,
            point,
            eyev,
            normalv: if inside { -normalv } else { normalv },
            inside,
        }
    }
    fn shade_hit(&self, comps: &Computations) -> Color {
        let mut result = BLACK;
        let material = self.objects[comps.object_id].get_material();
        for light in &self.lights {
            let color = lighting(material, &light, &comps.point, &comps.eyev, &comps.normalv);
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
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::approx_eq::{assert_approx_eq, ApproxEq};
    use crate::color::WHITE;
    use crate::transform::scaling;

    fn default_world() -> World {
        let mut world = World::new();
    
        world.add_light(PointLight::new(Point::new(-10.0, 10.0, -10.0), WHITE));
    
        let s1 = world.add_sphere();
        let mut m1 = Material::new();
        m1.color = Color::new(0.8, 1.0, 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;
        world.set_material(s1, m1);
    
        let s2 = world.add_sphere();
        world.set_transform(s2, scaling(0.5, 0.5, 0.5));
    
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
        let shape = world.add_sphere();
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
        let shape = world.add_sphere();
        let i = Intersection::new(4.0, shape);
        let comp = world.prepare_computations(&i, &r);
        assert!(!comp.inside);
    }

    #[test]
    fn test_the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut world = World::new();
        let shape = world.add_sphere();
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
        w.add_light(PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0)));
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
        let mut w = default_world();
        let mut m0 = Material::new();
        m0.ambient = 1.0;
        w.set_material(0, m0);
        let mut m1 = Material::new();
        m1.ambient = 1.0;
        let m1color = m1.color.clone();
        w.set_material(1, m1);
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_approx_eq!(c, m1color);
    }
}
