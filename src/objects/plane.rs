use crate::{vec::Vec3, scene::ObjectRay, ray::Ray, image::RGB};

pub struct Plane {
    pos: Vec3,
    dir: Vec3,
    color: RGB,
}

impl Plane{
    pub fn new(pos: Vec3, dir: Vec3, color: RGB) -> Self {
        Self {
            pos, dir, color
        }
    }
}

impl ObjectRay for Plane {
    fn bonce(&self, ray: &Ray) -> Option<RGB> {
        let point = self.intersect(ray)?;
        let distance = (point-ray.pos).abs2();
        let factor = 1000000.0/distance;
        Some(RGB{
            r: ((self.color.r as f32)*factor) as u8,
            g: ((self.color.g as f32)*factor) as u8,
            b: ((self.color.b as f32)*factor) as u8,
        })
    }

    fn intersect(&self, ray: &Ray) -> Option<Vec3> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
        let n = self.dir;
        let p0 = self.pos;

        let l0 = ray.pos;
        let l = ray.dir;

        let denom = l.dot(n);

        if denom >= 0.0 {
            None
        } else {
            let d = (p0-l0).dot(n)/denom;
            Some(l0+d*l)
        }
    }
}