use std::{f32::consts::PI};

use crate::{vec::Vec3, ray::Ray};

pub struct Camera {
    pub pos: Vec3,
    pub rotation_y: f32,
    pub rotation_x: f32,
    pub height: i32,
    pub width: i32,
    pub fov: f32,
}

impl Camera {
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width > 0 && height > 0, "Width and height whould be > 0 but are ({} {})", width, height);
        Camera {
            height, width, fov: PI*0.7,
            pos: Vec3::zero(),
            rotation_x: 0.0,
            rotation_y: 0.0,
        }
    }
    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let x = x/self.width as f32 - 0.5;
        let y = y/self.height as f32 - 0.5;

        let n = Vec3::new(1.0, 0.0, 0.0).rotate_z(self.rotation_x).rotate_y(self.rotation_y);
        let p = self.pos;

        let z = Vec3::new(0.0, 1.0, 0.0);

        let vy = (z - n*z.dot(n)).normalized();
        let vx = n.cross(vy).normalized();

        let pos = p + x*vx + y*vy;
        let dir = n.rotate_z(self.rotation_x-self.fov/2.0*y).rotate_y(self.rotation_y-self.fov/2.0*x);

        Ray {
            dir,
            pos
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vec3;

    use super::Camera;

    #[test]
    fn camera_default() {
        let camera = Camera::new(100, 100);

        for i in 0..100{
            for j in 0..100 {
                let expected = Vec3::new(0.0, -0.5 + i as f32/100.0, -0.5 + j as f32/100.0);
        
                assert_eq!(expected, camera.get_ray(j as f32, i as f32).pos, "{} {}", i, j);
            }
        }
    }
}