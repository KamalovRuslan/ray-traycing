use crate::ray::{HitRecord, Ray};
use crate::vector::Vec3;

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub trait Material {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hr: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hr.p + hr.normal + Vec3::random_in_unit_sphere();
        let reflected = Ray::new(hr.p, target - hr.p);
        Some((reflected, self.albedo))
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(r.direction().normalized(), hr.normal);
        let scattered = Ray::new(hr.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        if scattered.direction().dot(hr.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
