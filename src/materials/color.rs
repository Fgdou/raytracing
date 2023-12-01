use crate::{image::RGB, ray::Ray, vec::Vec3, scene::Scene};

use super::Material;

pub struct NormalColor {
}

impl Material for NormalColor {
    fn get_color(&self, ray: &Ray, normal: &Ray, scene: &Scene, bounce: i32) -> RGB {
        RGB{
            r: (normal.dir.x*255.0) as u8,
            g: (normal.dir.y*255.0) as u8,
            b: (normal.dir.z*255.0) as u8,
        }
    }
}

pub struct Color {
    pub rgb: RGB
}
impl Material for Color {
    fn get_color(&self, ray: &Ray, normal: &Ray, scene: &Scene, bounce: i32) -> RGB {
        self.rgb.clone()
    }
}