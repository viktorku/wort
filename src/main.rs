#![allow(clippy::needless_return)]

use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use num::clamp;
use rand::random;

mod vec3;
use vec3::{Color, Point3, Vec3};

mod ray;
use ray::Ray;

mod hit;
use hit::{HitRecord, Hittable};

mod hittable_list;
use hittable_list::HittableList;

mod sphere;
use sphere::Sphere;

mod camera;
use camera::Camera;

fn ray_color(ray: Ray, world: &dyn Hittable) -> Color {
    let mut record: HitRecord = Default::default();
    if world.hit(&ray, 0., f64::INFINITY, &mut record) {
        let n = record.normal.unwrap();
        return 0.5 * (n + Color::new(1., 1., 1.));
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_color(file: &mut File, color: &mut Color, samples_per_pixel: u8) -> std::io::Result<()> {
    // Divide the color by the number of samples to get the average
    *color /= samples_per_pixel as f64;
    writeln!(
        file,
        "{} {} {}",
        // Clamp if the average exceeds limits and convert to RGB scale
        (255. * clamp(color.x, 0., 1.)) as u8,
        (255. * clamp(color.y, 0., 1.)) as u8,
        (255. * clamp(color.z, 0., 1.)) as u8
    )?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    let mut file = File::create("image.ppm")?;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    // Image
    let aspect_ratio: f64 = 16. / 9.;
    let image_width: u16 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u16;
    let samples_per_pixel = 100;

    // Camera
    let cam = Camera::new(aspect_ratio);

    // Render

    writeln!(file, "P3")?;
    writeln!(file, "{} {}", image_width, image_height)?;
    writeln!(file, "255")?;

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        stdout.flush()?;

        for i in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;

                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(ray, &world);
            }
            write_color(&mut file, &mut pixel_color, samples_per_pixel)?;
        }
    }
    file.flush()?;

    Ok(())
}