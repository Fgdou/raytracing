use cgmath::{Vector3, InnerSpace};

use crate::{materials::Material, scene::ObjectRay, ray::Ray};

use super::triangle::Triangle;

pub struct Rectangle {
    material: Material,
    pos: Vector3<f32>,
    size: f32,
    normal: Vector3<f32>,
    triangles: [Triangle; 2]
}

impl Rectangle {
    pub fn new(center: Vector3<f32>, direction: Vector3<f32>, size: f32, material: Material) -> Self {
        let direction = direction.normalize();
        let top = Vector3::new(0.0, 1.0, 0.0);
        let x = (top.cross(direction)).normalize();
        let y = direction.cross(x);

        let p1 = center - x*size - y*size;
        let p2 = center - x*size + y*size;
        let p3 = center + x*size + y*size;
        let p4 = center + x*size - y*size;

        Self {
            material: material.clone(),
            normal: direction,
            pos: center,
            size,
            triangles: [
                Triangle::new([
                    p1, p2, p3
                ], material.clone()),
                Triangle::new([
                    p3, p4, p1
                ], material)
            ]
        }
    }
}

impl ObjectRay for Rectangle {
    fn intersect(&self, ray: &crate::ray::Ray) -> Option<Ray> {
        match (self.triangles[0].intersect(ray), self.triangles[1].intersect(ray)) {
            (Some(r), _) => Some(r),
            (_ , Some(r)) => Some(r),
            _ => None
        }
    }

    fn get_material(&self) -> &Material {
        return &self.material
    }
}