#![allow(dead_code)]

mod image;
mod vec;
mod camera;
mod ray;
mod objects;
mod scene;

use std::f32::consts::PI;

use image::{Image, ImageType, RGB};
use objects::{sphere::Sphere, plane::Plane};
use scene::{Scene, Material};
use vec::Vec3;

fn main() {
    let size = 1000;
    let mut image = Image::new(size, size);

    let mut scene = Scene::new(size as i32, size as i32);
    scene.get_camera().pos = Vec3::new(0.0, 10.0, 0.0);
    // scene.get_camera().rotation_x = -PI/8.0;
    scene.get_camera().rotation_y = -PI/8.0;

    for i in 0..5 {
        for j in 0..5 {
            let size = if i == 2 && j == 2 {10.0} else {3.0};

            let material = if i == 2 && j == 2 {
                Material::Mirror
            } else {
                Material::Color(RGB{
                    r: (i*255/5) as u8,
                    g: (j*255/5) as u8,
                    b: ((i+j)*255/10) as u8,
                })
            };

            scene.add_object(Box::from(Sphere::new(material, 
                size, 
                Vec3::new(
                    i as f32/5.0*100.0 + 50.0,
                    size,
                    j as f32/5.0*100.0 + 50.0,
                )
            )));
        }
    }
    scene.add_object(Box::from(Plane::new(
        Vec3::zero(),
        Vec3::new(0.0, 1.0, 0.0),
        RGB::new(255, 255, 255)
    )));

    scene.draw(&mut image);

    image.write_file("out.ppm", ImageType::Binary)
}
