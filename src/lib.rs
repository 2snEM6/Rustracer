pub mod scene;
pub mod sphere;
pub mod plane;
pub mod point;
pub mod color;
pub mod ray;
pub mod vector;
pub mod intersectable;




use image::{DynamicImage, Rgba, Pixel, GenericImage};
use scene::Scene;
use scene::Intersection;
use crate::ray::Ray;
use crate::intersectable::Intersectable;
use std::f64::INFINITY;


fn min(one: f64, another: f64) -> f64 {
    if one < another {
        one;
    }

    another
}


pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black_pixel = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            let plane_intersection_distance = scene.plane.intersect(&ray).unwrap_or(INFINITY);
            let sphere_intersection_distance = scene.sphere.intersect(&ray).unwrap_or(INFINITY);


            let minimum_distance = plane_intersection_distance.min(sphere_intersection_distance);

            if minimum_distance != INFINITY {
                if minimum_distance == plane_intersection_distance {
                    image.put_pixel(x, y, scene.plane.color.to_rgba());
                    println!("plane intersect");
                } else if minimum_distance == sphere_intersection_distance {
                    image.put_pixel(x, y, scene.sphere.color.to_rgba());
                    println!("sphere intersect");
                } else {
                    println!("black pixel");
                    image.put_pixel(x, y, black_pixel);
                }
            } else {
                println!("black pixel");
                image.put_pixel(x, y, black_pixel);
            }
//
//            let plane_intersection = Intersection::new(plane_intersection_distance.unwrap(), &scene.plane);
//            let sphere_intersection = Intersection::new(sphere_intersection_distance.unwrap(), &scene.sphere);
        }
    }
    image
}