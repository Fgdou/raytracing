use cgmath::{Vector3, InnerSpace};

use crate::{materials::Material, scene::ObjectRay, ray::Ray};

use super::plane::Plane;

pub struct Triangle {
    points: [Vector3<f32>; 3],
    material: Material,
    plane: Plane,
    normal: Vector3<f32>
}

impl Triangle {
    pub fn new(points: [Vector3<f32>; 3], material: Material) -> Self {

        let v1 = points[0] - points[1];
        let v2 = points[0] - points[2];
        let n = -(v1.cross(v2)).normalize();

        let plane = Plane::new(
            points[0],
            n,
            material.clone()
        );

        Self {
            points, 
            material,
            plane,
            normal: n,
        }
    }
}

impl ObjectRay for Triangle {
    fn intersect(&self, ray: &crate::ray::Ray) -> Option<Ray> {
        let point = self.plane.intersect(ray)?;

        let edge0 = self.points[1] - self.points[0];
        let edge1 = self.points[2] - self.points[1];
        let edge2 = self.points[0] - self.points[2];

        let line0 = point.pos - self.points[0];
        let line1 = point.pos - self.points[1];
        let line2 = point.pos - self.points[2];

        let n = -self.normal;

        if n.dot(edge0.cross(line0)) > 0.0 && n.dot(edge1.cross(line1)) > 0.0 && n.dot(edge2.cross(line2)) > 0.0 {
            Some(point)
        } else {
            None
        }
    }

    fn get_material(&self) -> &Material {
        return &self.material
    }
}