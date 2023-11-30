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
    scene.get_camera().pos = Vec3::new(-70.0, 30.0, 70.0);
    scene.get_camera().rotation_x = -0.1;
    scene.get_camera().rotation_y = 0.25;

    for i in 0..5 {
        for j in 0..5 {
            let size = 3.0;
            scene.add_object(Box::from(Sphere::new(RGB{
                    r: (i*255/5) as u8,
                    g: (j*255/5) as u8,
                    b: ((i+j)*255/10) as u8,
                }, 
                size, 
                Vec3::new(
                    i as f32/5.0*100.0 + 30.0,
                    size,
                    j as f32/5.0*100.0 - 40.0,
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
