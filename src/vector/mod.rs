use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn r(&self) -> f64 {
        self.x
    }
    pub fn g(&self) -> f64 {
        self.y
    }
    pub fn b(&self) -> f64 {
        self.z
    }
}

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Vec3 {
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -self.x * other.z + self.z * other.x,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Vec3 {
    pub fn normalized(&self) -> Self {
        *self / self.length()
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
            let p = p * 2. - Vec3::new(1., 1., 1.);
            if p.length_squared() < 1. {
                break p;
            }
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Self {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coor_test() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.x(), 1.);
        assert_eq!(v.y(), 2.);
        assert_eq!(v.z(), 3.);
        assert_eq!(v.r(), 1.);
        assert_eq!(v.g(), 2.);
        assert_eq!(v.b(), 3.);
    }
    #[test]
    fn len_test() {
        let v = Vec3::new(0., 3., 4.);
        assert_eq!(v.length_squared(), 25.);
        assert_eq!(v.length(), 5.);
    }
    #[test]
    fn normalized_test() {
        let v = Vec3::new(0., 3., 4.);
        let u = v.normalized();
        assert_eq!(u, Vec3::new(0., 0.6, 0.8));
        assert_eq!(v, Vec3::new(0., 3., 4.));
    }
    #[test]
    fn neg_test() {
        let v = Vec3::new(1., -2., 3.);
        assert_eq!(-v, Vec3::new(-1., 2., -3.));
    }
    #[test]
    fn scalar_left_mul_test() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(2.0 * v, Vec3::new(2., 4., 6.));
        assert_eq!(2.0 * v, v * 2.0);
    }
    #[test]
    fn operation_test() {
        let v = Vec3::new(1., 2., 3.);
        let u = Vec3::new(2., 3., 4.);
        assert_eq!(v * u, Vec3::new(2., 6., 12.));
        assert_eq!(v.cross(u), Vec3::new(-1., 2., -1.));
    }
    #[test]
    fn dot_test() {
        let v = Vec3::new(1., 2., 3.);
        let u = Vec3::new(-1., 0., 1.);
        assert_eq!(u.dot(v), 2.);
    }
    #[test]
    fn random_in_unit_sphere_test() {
        let v = Vec3::random_in_unit_sphere();
        assert!(v.x < 1.);
        assert!(v.x >= -1.);

        assert!(v.y < 1.);
        assert!(v.x >= -1.);

        assert!(v.z < 1.);
        assert!(v.x >= -1.);

        assert!(v.length_squared() < 1.);
    }
}
