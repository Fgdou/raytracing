use crate::vec::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}