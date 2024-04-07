mod vector;
mod ray;

use vector::Vec3;
use ray::Ray;

fn main() {
    let v = Vec3::new(0.1, 0.2, 0.3);
    println!("{}", v);
    let r = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1.,  1., 1.));
}
