use crate::{scene::{ObjectRay, RGBD, Material, Scene}, vec::Vec3, ray::Ray, image::RGB};

pub struct Sphere {
    material: Material,
    size: f32,
    pos: Vec3,
}

impl ObjectRay for Sphere {
    fn bonce(&self, ray: &Ray, scene: &Scene, bounce: i32) -> Option<RGBD> {
        let point = self.intersect(ray)?;
        let distance = (point - ray.pos).abs();
        let normal = (point - self.pos).normalized();

        match &self.material {
            Material::Color(c) => Some(RGBD{rgb: RGB{
                r: (normal.x*255.0) as u8,
                g: (normal.y*255.0) as u8,
                b: (normal.z*255.0) as u8,
            }, distance}),
            Material::Mirror => {
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
                
                Some(res)
            }
        }
    }
    fn intersect(&self, ray: &Ray) -> Option<Vec3> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection

        let c = self.pos;
        let r = self.size;
        let o = ray.pos;
        let u = ray.dir;

        let distance = o-c;
        let delta = u.dot(distance).powi(2) - (distance.abs2()-r*r);

        if delta <= 0.0 {
            None
        } else {
            let d = -(u.dot(distance));
            if d < 0.0 {
                None
            } else if delta == 0.0 {
                Some(o + d*u)
            } else {
                Some(o + (d-delta.sqrt())*u)
            }
        }
    }
}

impl Sphere {
    pub fn new(material: Material, size: f32, pos: Vec3) -> Self {
        Self {
            material, size, pos
        }
    }
}