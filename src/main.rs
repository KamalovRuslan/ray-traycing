mod camera;
mod material;
mod ray;
mod vector;

use camera::Camera;
use material::{Lambertian, Metal};
use ray::{color, HitList, Sphere};
use vector::Vec3;

use image::{Rgb, RgbImage};
use rand::prelude::*;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mat_ground = Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    };
    let mat_center = Lambertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    };
    let mat_left = Metal {
        albedo: Vec3::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    };
    let mat_right = Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    let world = HitList {
        hlist: vec![
            Sphere {
                center: Vec3::new(0., 0., -1.),
                r: 0.5,
                material: &mat_center,
            },
            Sphere {
                center: Vec3::new(0., -100.5, -1.),
                r: 100.,
                material: &mat_ground,
            },
            Sphere {
                center: Vec3::new(-1., 0., -1.),
                r: 0.5,
                material: &mat_left,
            },
            Sphere {
                center: Vec3::new(1., 0., -1.),
                r: 0.5,
                material: &mat_right,
            },
        ],
    };

    let camera = Camera::new();
    let mut image_buffer = RgbImage::new(nx as u32, ny as u32);
    let mut rng = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = camera.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= ns as f64;
            col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let ir = (255.99 * col.r()) as u8;
            let ig = (255.99 * col.g()) as u8;
            let ib = (255.99 * col.b()) as u8;
            image_buffer.put_pixel(i as u32, (ny - j - 1) as u32, Rgb([ir, ig, ib]));
        }
    }
    if let Err(err) = image_buffer.save("output.png") {
        eprintln!("Error saving image: {}", err);
    } else {
        println!("Image saved successfully!");
    }
}
