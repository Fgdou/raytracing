#![allow(dead_code)]

mod image;
mod vec;
mod camera;
mod ray;
mod objects;
mod scene;
use image::{Image, ImageType, RGB};
use objects::sphere::Sphere;
use scene::Scene;
use vec::Vec3;

fn main() {
    let size = 1000;
    let mut image = Image::new(size, size);

    let mut scene = Scene::new(size as i32, size as i32);

    let ball = Box::from(Sphere::new(
        RGB::new(255, 0, 0), 
        1.0, 
        Vec3::new(10.0, 0.0, 0.0)
    ));
    scene.add_object(ball);

    scene.draw(&mut image);

    image.write_file("out.ppm", ImageType::Binary)
}
