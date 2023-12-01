use crate::{ray::Ray, vec::Vec3, scene::Scene, image::RGB};

use super::Material;

pub struct Mirror {}

impl Material for Mirror {
    fn get_color(&self, ray: &Ray, normal: &Ray, scene: &Scene, bounce: i32) -> RGB {
        let point = normal.pos;
        let normal = normal.dir;

        let distance = (point - ray.pos).abs();

        let a = ray.pos - point;
        let b = (a.dot(normal)) * normal;
        let c = b-a;
        let d = (b+c).normalized();

        let mut res = scene.launch_ray(Ray{dir: d, pos: point}, bounce);
        res.distance = distance;

        let reduction = 1.0-0.04;
        res.rgb.r = (res.rgb.r as f32 * reduction) as u8;
        res.rgb.g = (res.rgb.g as f32 * reduction) as u8;
        res.rgb.b = (res.rgb.b as f32 * reduction) as u8;
        
        res.rgb
    }
}