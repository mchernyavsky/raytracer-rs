use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.dot(*self)
    }
}

trait Dot<Rhs = Self> {
    type Output;

    fn dot(self, other: Rhs) -> Self::Output;
}

impl Dot for Vec3 {
    type Output = f32;

    fn dot(self, other: Self) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

trait Cross<Rhs = Self> {
    type Output;

    fn cross(self, rhs: Rhs) -> Self::Output;
}

impl Cross for Vec3 {
    type Output = Self;

    fn cross(self, other: Self) -> Self::Output {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let Vec3 { x, y, z } = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);
    }

    #[test]
    fn test_getters() {
        let i = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(i.x(), 1.0);
        assert_eq!(i.y(), 2.0);
        assert_eq!(i.z(), 3.0);
    }

    #[test]
    fn test_length() {
        let i = Vec3::new(-2.0, -3.0, -5.0);
        let length = ((-2.0f32 * -2.0) + (-3.0 * -3.0) + (-5.0 * -5.0)).sqrt();
        assert_eq!(i.length(), length);
        assert_eq!((i * 2.0).length(), 2.0 * length);
        assert_eq!(i.length(), i.squared_length().sqrt());
    }

    #[test]
    fn test_squared_length() {
        let i = Vec3::new(-2.0, -3.0, -5.0);
        let squared_length = (-2.0 * -2.0) + (-3.0 * -3.0) + (-5.0 * -5.0);
        assert_eq!(i.squared_length(), squared_length);
        assert_eq!((i * 2.0).squared_length(), 4.0 * squared_length);
        assert_eq!(i.squared_length(), i.dot(i));
    }

    #[test]
    fn test_dot_product() {
        let i = Vec3::new(-2.0, -4.0, -8.0);
        let j = Vec3::new(-3.0, -9.0, -27.0);
        let dot = (-2.0 * -3.0) + (-4.0 * -9.0) + (-8.0 * -27.0);
        assert_eq!(i.dot(j), dot);
        assert_eq!((i * 2.0).dot(j * 2.0), 4.0 * dot);
    }

    #[test]
    fn test_cross_product() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(i.cross(j), k);
        assert_eq!(j.cross(k), i);
        assert_eq!(k.cross(i), j);
        assert_eq!(j.cross(i), -k);
        assert_eq!(k.cross(j), -i);
        assert_eq!(i.cross(k), -j);
        assert_eq!((i * 2.0).cross(j * 2.0), k * 4.0);
        assert_eq!((j * 2.0).cross(k * 2.0), i * 4.0);
        assert_eq!((k * 2.0).cross(i * 2.0), j * 4.0);
    }

    #[test]
    fn test_neg() {
        let i = Vec3::new(0.0, 1.0, -1.0);
        let j = -i;
        assert_eq!(j.x(), 0.0);
        assert_eq!(j.y(), -1.0);
        assert_eq!(j.z(), 1.0);
    }

    #[test]
    fn test_add() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = i + j;
        assert_eq!(k.x(), 1.0);
        assert_eq!(k.y(), 1.0);
        assert_eq!(k.z(), 0.0);
    }

    #[test]
    fn test_add_assign() {
        let mut i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        i += j;
        assert_eq!(i.x(), 1.0);
        assert_eq!(i.y(), 1.0);
        assert_eq!(i.z(), 0.0);
    }

    #[test]
    fn test_sub() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = i - j;
        assert_eq!(k.x(), 1.0);
        assert_eq!(k.y(), -1.0);
        assert_eq!(k.z(), 0.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        i -= j;
        assert_eq!(i.x(), 1.0);
        assert_eq!(i.y(), -1.0);
        assert_eq!(i.z(), 0.0);
    }

    #[test]
    fn test_mul() {
        let i = Vec3::new(-2.0, -3.0, 1.0);
        let j = Vec3::new(4.0, -5.0, 0.0);
        let k = i * j;
        assert_eq!(k.x(), -8.0);
        assert_eq!(k.y(), 15.0);
        assert_eq!(k.z(), 0.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut i = Vec3::new(-2.0, -3.0, 1.0);
        let j = Vec3::new(4.0, -5.0, 0.0);
        i *= j;
        assert_eq!(i.x(), -8.0);
        assert_eq!(i.y(), 15.0);
        assert_eq!(i.z(), 0.0);
    }

    #[test]
    fn test_mul_scalar() {
        let i = Vec3::new(1.0, -2.0, 0.0);
        let j = i * 2.0;
        let k = 2.0 * i;
        assert_eq!(j.x(), 2.0);
        assert_eq!(j.y(), -4.0);
        assert_eq!(j.z(), 0.0);
        assert_eq!(k.x(), 2.0);
        assert_eq!(k.y(), -4.0);
        assert_eq!(k.z(), 0.0);
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut i = Vec3::new(1.0, -2.0, 0.0);
        i *= 2.0;
        assert_eq!(i.x(), 2.0);
        assert_eq!(i.y(), -4.0);
        assert_eq!(i.z(), 0.0);
    }

    #[test]
    fn test_div_scalar() {
        let i = Vec3::new(2.0, -4.0, 0.0);
        let j = i / 2.0;
        assert_eq!(j.x(), 1.0);
        assert_eq!(j.y(), -2.0);
        assert_eq!(j.z(), 0.0);
    }

    #[test]
    fn test_div_assign_scalar() {
        let mut i = Vec3::new(2.0, -4.0, 0.0);
        i /= 2.0;
        assert_eq!(i.x(), 1.0);
        assert_eq!(i.y(), -2.0);
        assert_eq!(i.z(), 0.0);
    }
}
