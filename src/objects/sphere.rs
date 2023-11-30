use crate::{scene::{ObjectRay, RGBD}, image::RGB, vec::Vec3, ray::Ray};

pub struct Sphere {
    color: RGB,
    size: f32,
    pos: Vec3,
}

impl ObjectRay for Sphere {
    fn bonce(&self, ray: &Ray) -> Option<RGBD> {
        let point = self.intersect(ray)?;
        let distance = (point - ray.pos).abs();

        Some(RGBD{rgb: self.color.clone(), distance})

    }
    fn intersect(&self, ray: &Ray) -> Option<Vec3> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection

        let c = self.pos;
        let r = self.size;
        let o = ray.pos;
        let u = ray.dir;

        let distance = o-c;
        let delta = u.dot(distance).powi(2) - (distance.abs2()-r*r);

        if delta < 0.0 {
            None
        } else {
            let d = -(u.dot(distance));
            if delta == 0.0 {
                Some(o + d*u)
            } else {
                Some(o + (d-delta.sqrt())*u)
            }
        }
    }
}

impl Sphere {
    pub fn new(color: RGB, size: f32, pos: Vec3) -> Self {
        Self {
            color, size, pos
        }
    }
}