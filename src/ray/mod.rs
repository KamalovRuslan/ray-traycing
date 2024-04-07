use crate::vector::Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray{
    a: Vec3,
    b : Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { a: a, b: b }

    }
    pub fn origin(&self)-> &Vec3 {
        &self.a
    }
    pub fn direction(&self)-> &Vec3 {
        &self.b
    }
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.a + self.b * t
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_origin(){
        let r = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1.,  1., 1.));
        let a = r.origin();
        assert_eq!(*a, Vec3::new(0., 0., 0.));
    }
    #[test]
    fn test_direction(){
        let r = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1.,  1., 1.));
        let a = r.direction();
        assert_eq!(*a, Vec3::new(1., 1., 1.));
    }
    fn test_point_at(){
        let r = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1.,  1., 1.));
        let p = r.point_at(2.);
        assert_eq!(p, Vec3::new(2.,  2., 2.));
    }
}