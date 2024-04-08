mod vector;
mod ray;

use vector::Vec3;
use ray::Ray;
use image::{Rgb, RgbImage};

fn main() {
    let nx = 200;
    let ny = 100;
    let lower_lewt_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., -0., -0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);
    let mut image_buffer = RgbImage::new(nx as u32, ny as u32);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(origin, lower_lewt_corner + horizontal * u + vertical * v);
            let color = r.color();
            let ir = (255.99 * color.r()) as u8;
            let ig = (255.99 * color.g()) as u8;
            let ib = (255.99 * color.b()) as u8;
            image_buffer.put_pixel(i as u32, j as u32, Rgb([ir, ig, ib]));
        }
    }
    if let Err(err) = image_buffer.save("output.png") {
        eprintln!("Error saving image: {}", err);
    } else {
        println!("Image saved successfully!");
    }

}
