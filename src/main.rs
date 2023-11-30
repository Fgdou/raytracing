#![allow(dead_code)]

mod image;
mod vec;
mod camera;
mod ray;
mod mat3;
mod scene;
use image::{Image, RGB, ImageType};
use scene::Scene;

fn main() {
    let size = 1000;
    let mut image = Image::new(size, size);

    let scene = Scene::new(size as i32, size as i32);

    scene.draw(&mut image);

    image.write_file("out.ppm", ImageType::Binary)
}
