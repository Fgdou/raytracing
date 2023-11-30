use indicatif::ProgressBar;

use crate::{camera::Camera, image::{Image, RGB}, ray::Ray};

pub trait ObjectRay {
    fn bonce(&self, ray: &Ray) -> Option<RGB>;
}

pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn ObjectRay>>,
}

impl Scene {
    pub fn new(width: i32, height: i32) -> Scene {
        Scene {
            camera: Camera::new(width, height),
            objects: Vec::new()
        }
    }

    pub fn get_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn add_object(&mut self, obj: Box<dyn ObjectRay>) {
        self.objects.push(obj);
    }

    pub fn draw(&self, image: &mut Image) {
        let bar = ProgressBar::new((self.camera.height*self.camera.width) as u64);

        for y in 0..self.camera.height {
            for x in 0..self.camera.width {
                bar.inc(1);

                let ray = self.camera.get_ray(x, y);

                let mut color = RGB::default();

                for object in &self.objects {
                    match object.bonce(&ray){
                        Some(c) => color = c,
                        _ => ()
                    }
                }

                image.set_pixel(x as usize, y as usize, color);
            }
        }
        bar.finish();
    }
}