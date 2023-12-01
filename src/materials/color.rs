use crate::{image::RGB, ray::Ray, scene::Scene};

use super::Material;

pub struct NormalColor {
}

impl Material for NormalColor {
    fn get_color(&self, _: &Ray, normal: &Ray, _: &Scene, _: i32) -> RGB {
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
    fn get_color(&self, _: &Ray, _: &Ray, _: &Scene, _: i32) -> RGB {
        self.rgb.clone()
    }
}