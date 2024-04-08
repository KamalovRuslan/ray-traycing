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

impl Ray {
    pub fn color(&self) -> Vec3 {
        if self.hit_sphere(&Vec3::new(0., 0., -1.), 0.5){
            Vec3::new(1., 0., 0.)
        } else{
            let unit_direction = self.direction().make_unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.);
            Vec3::new(1., 1., 1.) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.) * t
        }
    }
    pub fn hit_sphere(&self, center: &Vec3, r: f64) -> bool {
        let oc = *self.origin() - *center;
        let a = self.direction().dot(*self.direction());
        let b = 2. * self.direction().dot(oc);
        let c = oc.dot(oc) - r * r;
        b * b - 4. * a * c > 0.
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
    #[test]
    fn test_point_at(){
        let r = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1.,  1., 1.));
        let p = r.point_at(2.);
        assert_eq!(p, Vec3::new(2.,  2., 2.));
    }
    #[test]
    fn test_color(){
        let r = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1.,  1., 1.));
        let c = r.color();
        assert_eq!(c, Vec3::new(0.6056624327025936, 0.7633974596215561, 1.0));
    }
}