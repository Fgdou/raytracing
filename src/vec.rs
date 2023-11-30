use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};

use crate::mat3::Mat3;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = 1e-6;

        (self.x-other.x).abs() < epsilon && 
        (self.y-other.y).abs() < epsilon && 
        (self.z-other.z).abs() < epsilon
    }
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

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index should be between 0 and 2")
        }
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index should be between 0 and 2")
        }
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

impl Mul<Mat3> for Vec3 {
    type Output = Vec3;

    fn mul(self, m: Mat3) -> Self::Output {
        let mut result = Vec3::zero();

        for col in 0..3 {
            for lign in 0..3 {
                result[col] += m[lign][col] * self[lign];
            }
        }

        result
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        let mut result = Vec3::zero();

        for lign in 0..3 {
            for col in 0..3 {
                result[lign] += self[lign][col] * v[col];
            }
        }

        result
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
        assert_eq!(expected, b + a);
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

    #[test]
    fn mul_mat() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Mat3::new([
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
            [10.0, 11.0, 12.0],
        ]);
        let expected1 = Vec3::new(48.0, 54.0, 60.0);
        let expected2 = Vec3::new(32.0, 50.0, 68.0);

        assert_eq!(expected1, a*b);
        assert_eq!(expected2, b*a);
    }
}