use crate::material::Material;
use crate::vector::Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub struct Sphere<'a> {
    pub center: Vec3,
    pub r: f64,
    pub material: &'a dyn Material,
}

pub struct HitList<T: Hit> {
    pub hlist: Vec<T>,
}

pub trait Hit {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord<'_>>;
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { a: a, b: b }
    }
    pub fn origin(&self) -> &Vec3 {
        &self.a
    }
    pub fn direction(&self) -> &Vec3 {
        &self.b
    }
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.a + self.b * t
    }
}

pub fn color<T: Hit>(r: &Ray, world: &T, depth: u32) -> Vec3 {
    if depth >= 50 {
        return Vec3::new(0., 0., 0.);
    }
    match world.hit(r, 0.001, f64::MAX) {
        Some(hit) => match hit.material.scatter(r, &hit) {
            Some((scattered, attenuation)) => attenuation * color(&scattered, world, depth + 1),
            None => Vec3::new(0., 0., 0.),
        },
        None => {
            let unit_direction = r.direction().normalized();
            let t = 0.5 * (unit_direction.y() + 1.);
            Vec3::new(1., 1., 1.) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.) * t
        }
    }
}

impl<'a> Sphere<'a> {
    fn check_hit_branch(&self, r: &Ray, tmin: f64, tmax: f64, root: f64) -> Option<HitRecord<'_>> {
        if root < tmax && root > tmin {
            let p = r.point_at(root);
            let normal = (p - self.center) / self.r;
            Some(HitRecord {
                t: root,
                p: p,
                normal: normal,
                material: self.material,
            })
        } else {
            None
        }
    }
}

impl<'a> Hit for Sphere<'a> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord<'_>> {
        let oc = *r.origin() - self.center;
        let a = r.direction().dot(*r.direction());
        let b = 2. * r.direction().dot(oc);
        let c = oc.dot(oc) - self.r * self.r;
        let discr = b * b - 4. * a * c;

        if discr > 0. {
            let left_root = (-b - discr.sqrt()) / (2. * a);
            let left_branch = self.check_hit_branch(r, tmin, tmax, left_root);
            match left_branch {
                Some(hr) => Some(hr),
                None => {
                    let right_root = (-b + discr.sqrt()) / (2. * a);
                    self.check_hit_branch(r, tmin, tmax, right_root)
                }
            }
        } else {
            None
        }
    }
}

impl<T: Hit> Hit for HitList<T> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord<'_>> {
        let mut temp_rec: Option<HitRecord<'_>> = None;
        let mut closest_so_far = tmax;
        for hitable in self.hlist.iter() {
            match hitable.hit(r, tmin, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t;
                    temp_rec = Some(hit);
                }
                None => {}
            }
        }
        temp_rec
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_origin() {
        let r = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.));
        let a = r.origin();
        assert_eq!(*a, Vec3::new(0., 0., 0.));
    }
    #[test]
    fn test_direction() {
        let r = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.));
        let a = r.direction();
        assert_eq!(*a, Vec3::new(1., 1., 1.));
    }
    #[test]
    fn test_point_at() {
        let r = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.));
        let p = r.point_at(2.);
        assert_eq!(p, Vec3::new(2., 2., 2.));
    }
}
