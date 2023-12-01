use crate::{vec::Vec3, scene::ObjectRay, ray::Ray, materials::Material};

pub struct Plane {
    pos: Vec3,
    dir: Vec3,
    material: Box<dyn Material>
}

impl Plane{
    pub fn new(pos: Vec3, dir: Vec3, material: Box<dyn Material>) -> Self {
        Self {
            pos, dir: dir.normalized(), material
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