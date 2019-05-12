use crate::material::Scatter;
use crate::{Ray, Vec3};

pub struct HitRecord<'a> {
    t: f64,
    point: Vec3,
    normal: Vec3,
    material: &'a dyn Scatter,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, point: Vec3, normal: Vec3, material: &'a dyn Scatter) -> Self {
        Self {
            t,
            point,
            normal,
            material,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn material(&self) -> &dyn Scatter {
        self.material
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HitList {
    data: Vec<Box<dyn Hit>>,
}

impl HitList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<T: Hit + 'static>(&mut self, value: T) {
        self.data.push(Box::new(value))
    }
}

impl Hit for HitList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record = None;
        let mut closest_so_far = t_max;
        for elem in self.data.iter() {
            if let Some(temp_record) = elem.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                record = Some(temp_record);
            }
        }
        record
    }
}
