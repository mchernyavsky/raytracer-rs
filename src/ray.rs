use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let actual = Ray::new(origin, direction);
        let expected = Ray { origin, direction };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_at_parameter() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        let vec3 = ray.point_at_parameter(2.0);
        assert_eq!(vec3, Vec3::new(9.0, 12.0, 15.0));
    }
}
