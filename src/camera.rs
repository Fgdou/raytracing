use cgmath::{Vector3, InnerSpace, Zero};

use crate::ray::Ray;

pub struct Camera {
    pub height: i32,
    pub width: i32,
    pub max_bound: i32,
    pub pos: Vector3<f32>,
    pub dir: Vector3<f32>,
    pub near: f32,
}

impl Camera {
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width > 0 && height > 0, "Width and height whould be > 0 but are ({} {})", width, height);
        Camera {
            height, width,
            max_bound: 5,
            pos: Vector3::zero(),
            dir: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            near: 1.0,
        }
    }
    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let x = x/self.width as f32 - 0.5;
        let y = -(y/self.height as f32 - 0.5);

        // pos
        let n = self.dir.normalize();
        let z = Vector3{x: 0.0, y: 1.0, z: 0.0};

        let vx = n.cross(z).normalize();
        let vy = -n.cross(vx).normalize();

        let o = self.pos + n*self.near + vx*x + vy*y;

        let dir = (o-self.pos).normalize(); 

        Ray {
            dir,
            pos: o,
        }
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Vector3;

    use super::Camera;

    #[test]
    fn camera_default() {
        let camera = Camera::new(100, 100);

        for i in 0..100{
            for j in 0..100 {
                let expected = Vector3::new(0.0, 0.5 - i as f32/100.0, -0.5 + j as f32/100.0);
        
                assert_eq!(expected, camera.get_ray(j as f32, i as f32).pos, "{} {}", i, j);
            }
        }
    }
}