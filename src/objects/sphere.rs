use crate::{scene::ObjectRay, image::RGB, vec::Vec3};

pub struct Sphere {
    color: RGB,
    size: f32,
    pos: Vec3,
}

impl ObjectRay for Sphere {
    fn bonce(&self, ray: &crate::ray::Ray) -> Option<RGB> {
        let distance = self.pos - ray.pos;
        let point = ray.dir.dot(distance) * ray.dir;

        let distance = (point - self.pos).abs();
        if distance > self.size {
            None
        } else {
            Some(self.color.clone())
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