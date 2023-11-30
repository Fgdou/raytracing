use crate::{vec::Vec3, ray::Ray};

pub struct Camera {
    ray: Ray,
    height: i32,
    width: i32,
}

impl Camera {
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width > 0 && height > 0, "Width and height whould be > 0 but are ({} {})", width, height);
        Camera {
            ray: Ray{pos: Vec3::zero(), dir: Vec3::from(1.0).normalized()}, 
            height, width
        }
    }
    pub fn set_pos(&mut self, pos: Vec3) {
        self.ray.pos = pos;
    }
    pub fn set_dir(&mut self, dir: &Vec3) {
        self.ray.dir = dir.normalized();
    }
    // pub fn get_ray(x: i32, y: i32) -> Ray {

    // }
}