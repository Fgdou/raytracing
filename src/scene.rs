use crate::{camera::Camera, image::Image};

pub trait ObjectRay {
    
}

pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn ObjectRay>>,
}

impl Scene {
    pub fn new(width: i32, height: i32) -> Scene {
        Scene {
            camera: Camera::new(width, height),
            objects: Vec::new()
        }
    }

    pub fn get_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn add_object(&mut self, obj: Box<dyn ObjectRay>) {
        self.objects.push(obj);
    }

    pub fn draw(&self, image: &mut Image) {

    }
}