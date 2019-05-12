pub mod scene;
pub mod sphere;
pub mod point;
pub mod color;
pub mod ray;
pub mod vector;
pub mod intersectable;


use image::{DynamicImage, Rgba, Pixel, GenericImage};
use scene::Scene;
use crate::ray::Ray;
use crate::intersectable::Intersectable;


pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black_pixel = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, scene.sphere.color.to_rgba())
            } else {
                image.put_pixel(x, y, black_pixel);
            }
        }
    }
    image
}