use std::ops::{IndexMut, Index, Mul};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Mat3 {
    vec: [[f32; 3]; 3]
}
impl Mat3 {
    pub fn zero() -> Self {
        Self{vec: [[0.0; 3]; 3]}
    }
    pub fn new(vec: [[f32; 3]; 3]) -> Self {
        Self{vec}
    }
}

impl Index<usize> for Mat3 {
    type Output = [f32; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl IndexMut<usize> for Mat3{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}

impl Mul for Mat3 {
    type Output = Mat3;

    fn mul(self, m: Self) -> Self::Output {
        let mut result = Mat3::zero();

        for lign1 in 0..3 {
            for col2 in 0..3 {
                for x in 0..3 {
                    result[lign1][col2] += self[lign1][x] *  m[x][col2];
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul() {
        let a = Mat3::new([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0]
        ]);
        let b = Mat3::new([
            [10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0],
            [16.0, 17.0, 18.0]
        ]);
        let expected1 = Mat3::new([
            [84.0, 90.0, 96.0],
            [201.0, 216.0, 231.0],
            [318.0, 342.0, 366.0]
        ]);
        let expected2 = Mat3::new([
            [138.0, 171.0, 204.0],
            [174.0, 216.0, 258.0],
            [210.0, 261.0, 312.0]
        ]);

        assert_eq!(expected1, a*b);
        assert_eq!(expected2, b*a);
    }
}