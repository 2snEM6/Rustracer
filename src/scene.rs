use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::point::Point;
use crate::vector::Vector3;
use crate::color::Color;
use crate::intersectable::Intersectable;
use crate::plane::Plane;

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &s.color,
            Element::Plane(ref p) => &p.color,
        }
    }
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }
}


pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have a finite distance.");
        }

        Intersection {
            distance,
            element,
        }
    }
}

pub struct Light {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}


pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
    pub plane: Plane,
    pub light: Light,
}

//impl Scene {
//    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
//        self.spheres
//            .iter()
//            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
//            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
//    }
//}