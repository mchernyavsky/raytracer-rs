use crate::hit::HitRecord;
use crate::material::Scatter;
use crate::{Hit, Ray, Vec3};
use std::sync::Arc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Scatter>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> Arc<dyn Scatter> {
        Arc::clone(&self.material)
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
                    return Some(HitRecord::new(t, point, normal, self.material()));
                }
            }
        }
        None
    }
}
