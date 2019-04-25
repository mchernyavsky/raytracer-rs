use crate::{Ray, Vec3};

pub struct HitRecord {
    t: f64,
    point: Vec3,
    normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f64, point: Vec3, normal: Vec3) -> Self {
        Self { t, point, normal }
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
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HitList<'a> {
    data: Vec<Box<Hit + 'a>>,
}

impl<'a> HitList<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push<T: Hit + 'a>(&mut self, value: T) {
        self.data.push(Box::new(value))
    }
}

impl<'a> Hit for HitList<'a> {
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
