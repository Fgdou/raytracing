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
    pub fn get_ray(&self, x: i32, y: i32) -> Ray {
        let x = x as f32/self.width as f32 - 0.5;
        let y = y as f32/self.height as f32 - 0.5;

        let n = self.ray.dir;
        let p = self.ray.pos;

        let z = Vec3::new(0.0, 1.0, 0.0);

        let vy = (z - n*z.dot(n)).normalized();
        let vx = n.cross(vy).normalized();

        let pos = p + x*vx + y*vy;

        Ray {
            dir: n,
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
        let mut camera = Camera::new(100, 100);
        let dir = Vec3::new(1.0, 0.0, 0.0);
        camera.set_dir(&dir);

        for i in 0..100{
            for j in 0..100 {
                let expected = Vec3::new(0.0, -0.5 + i as f32/100.0, -0.5 + j as f32/100.0);
        
                assert_eq!(expected, camera.get_ray(j, i).pos, "{} {}", i, j);
            }
        }
    }
}