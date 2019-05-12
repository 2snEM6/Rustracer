extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate ray_tracer;
extern crate image;

use clap::{Arg, App};
use std::fs::{File, OpenOptions};
use ray_tracer::*;
use image::ImageFormat;
use ray_tracer::scene::Scene;
use ray_tracer::sphere::Sphere;
use ray_tracer::point::Point;
use ray_tracer::color::Color;

fn main() {
    let app = App::new("raytracer")
        .version("0.1")
        .author("Daniel Limia <limiaspasdaniel@gmail.com>")
        .about("Basic Raytracer");
//    let matches = app.get_matches();
//
//    let image_path = matches.value_of("image").unwrap();
//
    let scene = Scene {
        width: 800,
        height: 800,
        fov: 90.0,
        sphere: {
            Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0
                },
                radius: {
                    1.0
                },
                color: Color {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.5
                }
            }
        }

    };

    let image = render(&scene);

    image.save("./image.png");

//    let mut image_file =
//        OpenOptions::new().write(true).truncate(true).create(true).open(image_path).unwrap();
//    image.write_to(&mut image_file, ImageFormat::PNG).unwrap();
}