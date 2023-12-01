use indicatif::ProgressBar;
use rand::random;

use crate::{camera::Camera, image::{Image, RGB}, ray::Ray, vec::Vec3, materials::Material};

pub struct RGBD {
    pub rgb: RGB,
    pub distance: f32
}

pub trait ObjectRay {
    fn intersect(&self, ray: &Ray) -> Option<Ray>;
    fn get_material(&self) -> &dyn Material;
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

    pub fn launch_ray(&self, ray: Ray, n: i32) -> RGBD {
        let mut color = RGBD{rgb: RGB::default(), distance: f32::INFINITY};
        if n >= self.camera.max_bound {
            return color;
        }

        for object in &self.objects {
            match object.intersect(&ray){
                Some(c) => {
                    let rgb = object.get_material().get_color(&ray, &c, self, n+1);

                    let distance = (ray.pos - c.pos).abs2();
                    let dir = ray.dir.dot(c.dir);

                    if dir < 0.0 && distance > 0.1 && distance < color.distance {
                        color = RGBD{rgb: rgb, distance: distance};
                    }
                },
                _ => ()
            }
        }

        color
    }

    pub fn draw(&self, image: &mut Image) {
        let bar = ProgressBar::new((self.camera.height*self.camera.width) as u64);

        for y in 0..self.camera.height {
            for x in 0..self.camera.width {
                let x = x as f32;
                let y = y as f32;

                bar.inc(1);

                let mut colors: Vec<RGBD> = Vec::new();
                for _ in 0..self.camera.antialiasing {
                    let ray = self.camera.get_ray(x, y);

                    let delta = 0.02;

                    let ray = Ray{
                        pos: ray.pos + Vec3::new(random::<f32>()*delta, random::<f32>()*delta, random::<f32>()*delta),
                        dir: ray.dir
                    };

                    colors.push(self.launch_ray(ray, 0));
                }

                let mut r: i32 = 0;
                let mut g: i32 = 0;
                let mut b: i32 = 0;

                for c in &colors {
                    r += c.rgb.r as i32;
                    g += c.rgb.g as i32;
                    b += c.rgb.b as i32;
                }

                image.set_pixel(x as usize, y as usize, RGB{
                    r: (r/colors.len() as i32) as u8,
                    g: (g/colors.len() as i32) as u8,
                    b: (b/colors.len() as i32) as u8,
                });
            }
        }
        bar.finish();
    }
}