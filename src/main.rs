#![allow(dead_code)]

mod image;
mod vec;
mod camera;
mod ray;
mod objects;
mod scene;
use image::{Image, ImageType, RGB};
use objects::{sphere::Sphere, plane::Plane};
use scene::Scene;
use vec::Vec3;

fn main() {
    let size = 1000;
    let mut image = Image::new(size, size);

    let mut scene = Scene::new(size as i32, size as i32);
    scene.get_camera().pos.y = 10.0;

    scene.add_object(Box::from(Sphere::new(
        RGB::new(255, 0, 0), 
        1.0, 
        Vec3::new(10.0, 10.0, 0.0)
    )));
    scene.add_object(Box::from(Plane::new(
        Vec3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        RGB::new(255, 255, 255)
    )));

    scene.draw(&mut image);

    image.write_file("out.ppm", ImageType::Binary)
}
