use crate::hit::HitRecord;
use crate::{Ray, Vec3};
use rand::distributions::{Distribution, UnitSphereSurface};

pub struct ScatteredRay {
    ray: Ray,
    attenuation: Vec3,
}

impl ScatteredRay {
    pub fn new(ray: Ray, attenuation: Vec3) -> Self {
        ScatteredRay { ray, attenuation }
    }

    pub fn ray(&self) -> Ray {
        self.ray
    }

    pub fn attenuation(&self) -> Vec3 {
        self.attenuation
    }
}

pub trait Scatter: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let target = hit.point() + hit.normal() + random_in_unit_sphere();
        Some(ScatteredRay::new(
            Ray::new(hit.point(), target - hit.point()),
            self.albedo,
        ))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let reflected = reflect(ray.direction().normalize(), hit.normal());
        let scattered = Ray::new(hit.point(), reflected + self.fuzz * random_in_unit_sphere());
        if scattered.direction().dot(hit.normal()) > 0.0 {
            Some(ScatteredRay::new(scattered, self.albedo))
        } else {
            None
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let sphere = UnitSphereSurface::new();
    let [x, y, z] = sphere.sample(&mut rand::thread_rng());
    Vec3::new(x, y, z)
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}
