mod camera;
mod ray;
mod vector;

use camera::Camera;
use ray::{HitList, Sphere};
use vector::Vec3;

use image::{Rgb, RgbImage};
use rand::prelude::*;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let world = HitList {
        hlist: vec![
            Sphere {
                center: Vec3::new(0., 0., -1.),
                r: 0.5,
            },
            Sphere {
                center: Vec3::new(0., -100.5, -1.),
                r: 100.,
            },
        ],
    };
    let camera = Camera::new();
    let mut image_buffer = RgbImage::new(nx as u32, ny as u32);
    let mut rng = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::new(0., 0., 0.);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = camera.get_ray(u, v);
                color += r.color(&world);
            }
            color /= ns as f64;
            color = Vec3::new(color.x().sqrt(), color.y().sqrt(), color.z().sqrt());
            let ir = (255.99 * color.r()) as u8;
            let ig = (255.99 * color.g()) as u8;
            let ib = (255.99 * color.b()) as u8;
            image_buffer.put_pixel(i as u32, (ny - j - 1) as u32, Rgb([ir, ig, ib]));
        }
    }
    if let Err(err) = image_buffer.save("output.png") {
        eprintln!("Error saving image: {}", err);
    } else {
        println!("Image saved successfully!");
    }
}
