use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self{x, y, z}
    }
    pub fn zero() -> Self {
        Self{x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn abs2(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn abs(&self) -> f32 {
        f32::sqrt(self.abs2())
    }
    pub fn normalized(&self) -> Vec3 {
        let d = self.abs2();
        Vec3 {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }
}
impl From<f32> for Vec3 {
    fn from(value: f32) -> Self {
        Self{x: value, y: value, z: value}
    }
}

impl Add for Vec3  {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, n: f32) -> Self::Output {
        Vec3 {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, n: f32) -> Self::Output {
        Vec3 {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::from(8.0);

        let expected = Vec3::new(9.0, 10.0, 11.0);

        assert_eq!(expected, a + b);
    }

    #[test]
    fn sub() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::from(8.0);

        let expected = Vec3::new(-7.0, -6.0, -5.0);

        assert_eq!(expected, a - b);
    }

    #[test]
    fn div() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 8.0;

        let expected = Vec3::new(1.0/8.0, 2.0/8.0, 3.0/8.0);

        assert_eq!(expected, a / b);
    }

    #[test]
    fn mul() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 8.0;

        let expected = Vec3::new(1.0*8.0, 2.0*8.0, 3.0*8.0);

        assert_eq!(expected, a * b);
    }
}