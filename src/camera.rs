use std::{f32::consts::PI, process::exit};

use crate::{vec::Vec3, ray::Ray};

pub struct Camera {
    pub pos: Vec3,
    pub rotationY: f32,
    pub rotationX: f32,
    pub height: i32,
    pub width: i32,
    pub fov: f32,
}

impl Camera {
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width > 0 && height > 0, "Width and height whould be > 0 but are ({} {})", width, height);
        Camera {
            height, width, fov: PI/2.0,
            pos: Vec3::zero(),
            rotationX: 0.0,
            rotationY: 0.0,
        }
    }
    pub fn get_ray(&self, x: i32, y: i32) -> Ray {
        let x = x as f32/self.width as f32 - 0.5;
        let y = y as f32/self.height as f32 - 0.5;

        let n = Vec3::new(1.0, 0.0, 0.0).rotateZ(self.rotationX).rotateY(self.rotationY);
        let p = self.pos;

        let z = Vec3::new(0.0, 1.0, 0.0);

        let vy = (z - n*z.dot(n)).normalized();
        let vx = n.cross(vy).normalized();

        let pos = p + x*vx + y*vy;
        let dir = n.rotateZ(self.rotationX-self.fov/2.0*y).rotateY(self.rotationY-self.fov/2.0*x);

        Ray {
            dir,
            pos
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{vec::Vec3, ray::Ray};

    use super::Camera;

    #[test]
    fn camera_default() {
        let mut camera = Camera::new(100, 100);

        for i in 0..100{
            for j in 0..100 {
                let expected = Vec3::new(0.0, -0.5 + i as f32/100.0, -0.5 + j as f32/100.0);
        
                assert_eq!(expected, camera.get_ray(j, i).pos, "{} {}", i, j);
            }
        }
    }
}