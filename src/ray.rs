use crate::point::Point;
use crate::scene::Scene;
use crate::vector::Vector3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(pixel_x: u32, pixel_y: u32, scene: &Scene) -> Ray {
        assert!(scene.width >= scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = ((pixel_x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0 * aspect_ratio * fov_adjustment;
        let sensor_y = 1.0 - ((pixel_y as f64 + 0.5) / scene.height as f64) * 2.0 * fov_adjustment;

        Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            direction: Vector3 {
                x: sensor_x as f64,
                y: sensor_y as f64,
                z: -1.0,
            }
                .normalize(),
        }
    }
}