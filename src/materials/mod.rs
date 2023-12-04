use cgmath::InnerSpace;

use crate::{image::RGB, ray::Ray, scene::Scene};

pub enum Material {
    Color(RGB),
    Normal,
    Mirror
}

impl Material {
    pub fn get_color(&self, ray: &Ray, normal: &Ray, scene: &Scene, bounce: i32) -> RGB {
        match self {
            Material::Color(c) => c.clone(),
            Material::Normal => RGB{
                r: (255.0-normal.dir.x*255.0) as u8,
                g: (normal.dir.y*255.0) as u8,
                b: (normal.dir.z*255.0) as u8,
            },
            Material::Mirror => {
                let point = normal.pos;
                let normal = normal.dir;

                let distance = (point - ray.pos).magnitude();

                let a = ray.pos - point;
                let b = (a.dot(normal)) * normal;
                let c = b-a;
                let d = (b+c).normalize();

                let mut res = scene.launch_ray(Ray{dir: d, pos: point}, bounce);
                res.distance = distance;

                let reduction = 1.0-0.04;
                res.rgb.r = (res.rgb.r as f32 * reduction) as u8;
                res.rgb.g = (res.rgb.g as f32 * reduction) as u8;
                res.rgb.b = (res.rgb.b as f32 * reduction) as u8;
                
                res.rgb
            },
        }
    }
}