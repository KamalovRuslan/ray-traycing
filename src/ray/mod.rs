use crate::vector::Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

pub struct HitRecord {
    t: f64,
    p: Vec3,
    normal: Vec3,
}

pub struct Sphere {
    pub center: Vec3,
    pub r: f64,
}

pub struct HitList<T: Hit> {
    pub hlist: Vec<T>,
}

pub trait Hit {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
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

impl Ray {
    pub fn color<T: Hit>(&self, world: &T) -> Vec3 {
        match world.hit(&self, 0.0, f64::MAX) {
            Some(hit) => {
                Vec3::new(
                    hit.normal.x() + 1.,
                    hit.normal.y() + 1.,
                    hit.normal.z() + 1.,
                ) * 0.5
            }
            None => {
                let unit_direction = self.direction().make_unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.);
                Vec3::new(1., 1., 1.) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.) * t
            }
        }
    }
    pub fn hit_sphere(&self, center: &Vec3, r: f64) -> Option<f64> {
        let oc = *self.origin() - *center;
        let a = self.direction().dot(*self.direction());
        let b = 2. * self.direction().dot(oc);
        let c = oc.dot(oc) - r * r;
        let discr = b * b - 4. * a * c;
        if discr < 0. {
            None
        } else {
            Some((-b - discr) / (2. * a))
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = r.direction().dot(*r.direction());
        let b = 2. * r.direction().dot(oc);
        let c = oc.dot(oc) - self.r * self.r;
        let discr = b * b - 4. * a * c;
        let result = if discr > 0. {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            let inner_result_left = if temp < tmax && temp > tmin {
                let p = r.point_at(temp);
                let normal = (p - self.center) / self.r;
                Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: normal,
                })
            } else {
                None
            };
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            let inner_result_right = if temp < tmax && temp > tmin {
                let p = r.point_at(temp);
                let normal = (p - self.center) / self.r;
                Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: normal,
                })
            } else {
                None
            };
            match inner_result_left {
                Some(hr) => Some(hr),
                None => match inner_result_right {
                    Some(hr) => Some(hr),
                    None => None,
                },
            }
        } else {
            None
        };
        result
    }
}

impl<T: Hit> Hit for HitList<T> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
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
