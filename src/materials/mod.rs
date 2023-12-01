pub mod color;
pub mod mirror;

use crate::{image::RGB, ray::Ray, vec::Vec3, scene::Scene};

pub trait Material {
    fn get_color(&self, ray: &Ray, normal: &Ray, scene: &Scene, bounce: i32) -> RGB;
}