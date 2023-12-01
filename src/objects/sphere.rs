use cgmath::{Vector3, InnerSpace};

use crate::{scene::ObjectRay, ray::Ray, materials::Material};

pub struct Sphere {
    size: f32,
    pos: Vector3<f32>,
    material: Box<dyn Material>,
}

impl ObjectRay for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Ray> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection

        let c = self.pos;
        let r = self.size;
        let o = ray.pos;
        let u = ray.dir;

        let distance = o-c;
        let delta = u.dot(distance).powi(2) - (distance.magnitude2()-r*r);

        if delta <= 0.0 {
            None
        } else {
            let d = -(u.dot(distance));
            let point = if d < 0.0 {
                None
            } else if delta == 0.0 {
                Some(o + d*u)
            } else {
                Some(o + (d-delta.sqrt())*u)
            }?;
            let normal = (point-self.pos).normalize();

            Some(Ray {
                pos: point,
                dir: normal
            })
        }
    }

    fn get_material(&self) -> &dyn Material {
        self.material.as_ref()
    }
}

impl Sphere {
    pub fn new(size: f32, pos: Vector3<f32>, material: Box<dyn Material>) -> Self {
        Self {
            size, pos, material
        }
    }
}