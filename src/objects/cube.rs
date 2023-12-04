use cgmath::Vector3;

use crate::materials::Material;

pub struct Cube {
    pos: Vector3<f32>,
    material: Material,
    size: f32,
}

pub impl Cube {
    pub fn new(pos: Vector3<f32>, size: f32, material: Material) -> Self {
        Self{pos, material, size}
    }
}