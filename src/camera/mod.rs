use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);
        Camera {
            lower_left_corner: lookfrom - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: lookfrom,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
