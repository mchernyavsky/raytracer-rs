use crate::sphere::Sphere;
use crate::{Ray, Vec3};

pub struct HitRecord {
    t: f32,
    point: Vec3,
    normal: Vec3,
}

impl HitRecord {
    fn new(t: f32, point: Vec3, normal: Vec3) -> Self {
        Self { t, point, normal }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center();
        let a = ray.direction().squared_length();
        let b = oc.dot(ray.direction());
        let c = oc.squared_length() - self.radius() * self.radius();
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let d_sqrt = discriminant.sqrt();
            for &temp in &[(-b - d_sqrt) / a, (-b + d_sqrt) / a] {
                if temp < t_max && temp > t_min {
                    let t = temp;
                    let point = ray.point_at_parameter(t);
                    let normal = (point - self.center()) / self.radius();
                    return Some(HitRecord::new(t, point, normal));
                }
            }
        }
        None
    }
}
