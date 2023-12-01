#![allow(dead_code)]

mod image;
mod camera;
mod ray;
mod objects;
mod scene;
mod materials;

use cgmath::{Vector3, Zero};
use image::{Image, ImageType};
use materials::{color::{NormalColor, Color}, mirror::Mirror};
use objects::{sphere::Sphere, plane::Plane};
use scene::Scene;

fn main() {
    let size = 1000;
    let mut image = Image::new(size, size);

    let mut scene = Scene::new(size as i32, size as i32);
    scene.get_camera().pos = Vector3::new(0.0, 4.0, 0.0);
    scene.get_camera().dir = Vector3::new(1.0, 0.1, 1.0);

    for i in 0..11 {
        for j in 0..11 {
            let z = i as f32*2.0 + 10.0;
            let x = j as f32*2.0 + 10.0;

            let size = 1.0;
            let color = NormalColor{};

            scene.add_object(Box::new(Sphere::new(
                size, Vector3::new(x, 1.0, z), Box::new(color)
            )));
        }
    }

    scene.add_object(Box::new(Sphere::new(
        5.0,
        Vector3::new(30.0, 6.0, 30.0),
        Box::new(Mirror{})
    )));
    
    scene.add_object(Box::from(Plane::new(
        Vector3::zero(),
        Vector3::new(0.0, 1.0, 0.0),
        Box::from(Color{rgb: image::RGB { r: 255, g: 255, b: 255 }})
    )));

    scene.draw(&mut image);

    image.write_file("out.ppm", ImageType::Binary)
}
