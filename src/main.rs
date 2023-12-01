#![allow(dead_code)]

mod image;
mod camera;
mod ray;
mod objects;
mod scene;
mod materials;

use cgmath::Vector3;
use image::{Image, ImageType};
use materials::color::NormalColor;
use objects::sphere::Sphere;
use scene::Scene;

fn main() {
    let size = 1000;
    let mut image = Image::new(size, size);

    let mut scene = Scene::new(size as i32, size as i32);

    scene.add_object(Box::from(Sphere::new(
        1.0,
        Vector3 { x: 5.0, y: 0.0, z: 0.0 },
        Box::from(NormalColor{})
    )));

    scene.draw(&mut image);

    image.write_file("out.ppm", ImageType::Binary)
}
