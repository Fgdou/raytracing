use cgmath::Vector3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Ray {
    pub pos: Vector3<f32>,
    pub dir: Vector3<f32>,
}