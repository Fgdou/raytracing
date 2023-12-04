use cgmath::{Vector3, InnerSpace};

use crate::{materials::Material, scene::ObjectRay, ray::Ray};

use super::plane::Plane;

pub struct Triangle {
    points: [Vector3<f32>; 3],
    material: Material,
    plane: Plane
}

impl Triangle {
    pub fn new(points: [Vector3<f32>; 3], material: Material) -> Self {

        let v1 = points[0] - points[1];
        let v2 = points[0] - points[2];
        let n = (v1.cross(v2)).normalize();

        let plane = Plane::new(
            points[0],
            n,
            material.clone()
        );

        Self {
            points, 
            material,
            plane,
        }
    }
}

impl ObjectRay for Triangle {
    fn intersect(&self, ray: &crate::ray::Ray) -> Option<Ray> {
        let point = self.plane.intersect(ray)?;

        for i in 0..3 {
            if (point.pos-self.points[i]).magnitude2() > (self.points[(i+1)%3] - self.points[i]).magnitude2() && 
                (point.pos-self.points[i]).magnitude2() > (self.points[(i+2)%3] - self.points[i]).magnitude2() {
                    return None;
                }
        }
        Some(point)
    }

    fn get_material(&self) -> &Material {
        return &self.material
    }
}