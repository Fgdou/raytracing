use crate::{scene::ObjectRay, image::RGB, vec::Vec3, ray::Ray};

pub struct Sphere {
    color: RGB,
    size: f32,
    pos: Vec3,
}

impl ObjectRay for Sphere {
    fn bonce(&self, ray: &Ray) -> Option<RGB> {
        match self.intersect(ray) {
            Some(_) => Some(self.color.clone()),
            None => None
        }
    }
}

impl Sphere {
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
        } else if delta == 0.0 {
            Some(o + -(u.dot(distance))*u)
        } else {
            Some(o + (-(u.dot(distance))-delta)*u)
        }
    }
    pub fn new(color: RGB, size: f32, pos: Vec3) -> Self {
        Self {
            color, size, pos
        }
    }
}