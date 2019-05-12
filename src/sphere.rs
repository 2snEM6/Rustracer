use crate::point::Point;
use crate::color::Color;
use crate::ray::Ray;
use crate::intersectable::Intersectable;
use crate::vector::Vector3;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        //Create a line segment between the ray origin and the center of the sphere
        let ray_origin_to_sphere_center: Vector3 = self.center - ray.origin;
        //Use ray_origin_to_sphere_center as a hypotenuse and find the length of the adjacent side
        let adjacent_side_length = ray_origin_to_sphere_center.dot(&ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let distance = ray_origin_to_sphere_center.dot(&ray_origin_to_sphere_center) - (adjacent_side_length * adjacent_side_length);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        distance < (self.radius * self.radius)
    }
}