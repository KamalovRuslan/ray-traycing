use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Camera {
    lower_lewt_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            lower_lewt_corner: Vec3::new(-2., -1., -1.),
            horizontal: Vec3::new(4., -0., -0.),
            vertical: Vec3::new(0., 2., 0.),
            origin: Vec3::new(0., 0., 0.),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_lewt_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
