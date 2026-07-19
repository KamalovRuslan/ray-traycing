use crate::ray::{HitRecord, Ray};
use crate::vector::Vec3;
use rand::prelude::*;

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let cos_theta = -v.dot(n);
    let r_perp = ni_over_nt * (v + cos_theta * n);
    let r_perp_len_sq = r_perp.length_squared();
    if r_perp_len_sq > 1.0 {
        return None;
    }
    let r_parallel = -(1.0 - r_perp_len_sq).sqrt() * n;
    Some(r_perp + r_parallel)
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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

pub struct Dielectric {
    pub ref_idx: f64,
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

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Ray, Vec3)> {
        let direction = r.direction().normalized();
        let (outward_normal, ni_over_nt, cosine) = if direction.dot(hr.normal) > 0.0 {
            (-hr.normal, self.ref_idx, direction.dot(hr.normal))
        } else {
            (hr.normal, 1.0 / self.ref_idx, -direction.dot(hr.normal))
        };
        let reflected = reflect(direction, hr.normal);
        match refract(direction, outward_normal, ni_over_nt) {
            None => Some((Ray::new(hr.p, reflected), Vec3::new(1., 1., 1.))),
            Some(refracted) => {
                if rand::thread_rng().gen::<f64>() < schlick(cosine, self.ref_idx) {
                    Some((Ray::new(hr.p, reflected), Vec3::new(1., 1., 1.)))
                } else {
                    Some((Ray::new(hr.p, refracted), Vec3::new(1., 1., 1.)))
                }
            }
        }
    }
}
