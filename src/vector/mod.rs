use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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
    pub fn len(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn squared_len(&self) -> f64 {
        self.len().sqrt()
    }
}

impl Vec3 {
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Vec3 {
    pub fn unit_vector(&mut self) {
        *self /= self.squared_len();
    }
    pub fn make_unit_vector(&self) -> Self {
        *self / self.squared_len()
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
            x: self.y * other.z - self.z * other.y,
            y: -self.x * other.z + self.z * other.x,
            z: self.x * other.y - self.y * other.x,
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
        self.x = self.y * other.z - self.z * other.y;
        self.y = -self.x * other.z + self.z * other.x;
        self.z = self.x * other.y - self.y * other.x;
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
        assert_eq!(v.len(), 25.);
        assert_eq!(v.squared_len(), 5.);
    }
    #[test]
    fn unit_vector_test() {
        let mut v = Vec3::new(0., 3., 4.);
        v.unit_vector();
        assert_eq!(v, Vec3::new(0., 0.6, 0.8));

        let v = Vec3::new(0., 3., 4.);
        let u = v.make_unit_vector();
        assert_eq!(u, Vec3::new(0., 0.6, 0.8));
        assert_eq!(v, Vec3::new(0., 3., 4.));
    }
    #[test]
    fn operation_test() {
        let v = Vec3::new(1., 2., 3.);
        let u = Vec3::new(0., 1., 0.);
        assert_eq!(v * u, Vec3::new(-3., 0., 1.));
    }
    #[test]
    fn dot_test() {
        let v = Vec3::new(1., 2., 3.);
        let u = Vec3::new(-1., 0., 1.);
        assert_eq!(u.dot(v), 2.);
    }
}
