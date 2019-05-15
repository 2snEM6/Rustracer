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
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let ray_origin_to_sphere_center: Vector3 = self.center - ray.origin;
        let projection_on_ray_length = ray_origin_to_sphere_center.dot(&ray.direction);
        let ray_origin_to_sphere_center_length_squared =
            ray_origin_to_sphere_center.dot(&ray_origin_to_sphere_center);
        let distance_squared = ray_origin_to_sphere_center_length_squared - (projection_on_ray_length * projection_on_ray_length);

        let radius_squared = self.radius * self.radius;
        if distance_squared > radius_squared {
            return None;
        }

        let thc = (radius_squared - distance_squared).sqrt();
        let t0 = projection_on_ray_length - thc;
        let t1 = projection_on_ray_length + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }
}