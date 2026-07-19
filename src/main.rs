mod camera;
mod material;
mod ray;
mod vector;

use camera::Camera;
use material::{Dielectric, Lambertian, Material, Metal};
use ray::{color, HitList, Sphere};
use vector::Vec3;

use image::{Rgb, RgbImage};
use rand::prelude::*;
use rayon::prelude::*;

fn random_scene() -> HitList {
    let mut spheres: Vec<Sphere> = Vec::new();

    // ground
    spheres.push(Sphere {
        center: Vec3::new(0., -1000., 0.),
        r: 1000.,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
    });

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4., 0.2, 0.)).length() < 0.9 {
                continue;
            }

            let material: Box<dyn Material> = if choose_mat < 0.8 {
                let albedo = Vec3::new(
                    rng.gen::<f64>() * rng.gen::<f64>(),
                    rng.gen::<f64>() * rng.gen::<f64>(),
                    rng.gen::<f64>() * rng.gen::<f64>(),
                );
                Box::new(Lambertian { albedo })
            } else if choose_mat < 0.95 {
                let albedo = Vec3::new(
                    0.5 + 0.5 * rng.gen::<f64>(),
                    0.5 + 0.5 * rng.gen::<f64>(),
                    0.5 + 0.5 * rng.gen::<f64>(),
                );
                let fuzz = 0.5 * rng.gen::<f64>();
                Box::new(Metal { albedo, fuzz })
            } else {
                Box::new(Dielectric { ref_idx: 1.5 })
            };

            spheres.push(Sphere {
                center,
                r: 0.2,
                material,
            });
        }
    }

    // three big spheres
    spheres.push(Sphere {
        center: Vec3::new(0., 1., 0.),
        r: 1.0,
        material: Box::new(Dielectric { ref_idx: 1.5 }),
    });
    spheres.push(Sphere {
        center: Vec3::new(-4., 1., 0.),
        r: 1.0,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }),
    });
    spheres.push(Sphere {
        center: Vec3::new(4., 1., 0.),
        r: 1.0,
        material: Box::new(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    });

    HitList { hlist: spheres }
}

fn main() {
    let nx = 1200;
    let ny = 600;
    let ns = 100;

    let world = random_scene();

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let focus_dist = 10.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        20.0,
        nx as f64 / ny as f64,
        0.1,
        focus_dist,
    );

    let mut image_buffer = RgbImage::new(nx as u32, ny as u32);

    let pixels: Vec<(u32, u32, [u8; 3])> = (0..ny)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            let mut rng = rand::thread_rng();
            (0..nx)
                .map(|i| {
                    let col = (0..ns)
                        .map(|_| {
                            let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                            let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                            color(&camera.get_ray(u, v), &world, 0)
                        })
                        .fold(Vec3::new(0., 0., 0.), |acc, c| acc + c)
                        / ns as f64;
                    let col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
                    let ir = (255.99 * col.r()) as u8;
                    let ig = (255.99 * col.g()) as u8;
                    let ib = (255.99 * col.b()) as u8;
                    (i as u32, (ny - j - 1) as u32, [ir, ig, ib])
                })
                .collect::<Vec<_>>()
        })
        .collect();

    for (x, y, rgb) in pixels {
        image_buffer.put_pixel(x, y, Rgb(rgb));
    }

    if let Err(err) = image_buffer.save("output.png") {
        eprintln!("Error saving image: {}", err);
    } else {
        println!("Image saved successfully!");
    }
}
