use cgmath::{InnerSpace, Vector3};

use crate::{scene::ObjectRay, ray::Ray, materials::Material};

pub struct Plane {
    pos: Vector3<f32>,
    dir: Vector3<f32>,
    material: Box<dyn Material>
}

impl Plane{
    pub fn new(pos: Vector3<f32>, dir: Vector3<f32>, material: Box<dyn Material>) -> Self {
        Self {
            pos, dir: dir.normalize(), material
        }
    }
}

impl ObjectRay for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Ray> {
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
            let point = l0+d*l;

            Some(Ray {
                pos: point,
                dir: self.dir
            })
        }
    }

    fn get_material(&self) -> &dyn Material {
        self.material.as_ref()
    }
}