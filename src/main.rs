#![allow(clippy::needless_return)]

use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

use num::clamp;
use rand::random;

mod vec3;
use vec3::{Color, Vec3};

mod ray;
use ray::Ray;

mod hit;
use hit::Hittable;

mod hittable_list;
use hittable_list::HittableList;

mod sphere;
use sphere::Sphere;

mod camera;
use camera::Camera;

fn ray_color(ray: Ray, world: &dyn Hittable, ray_bounce: u8) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    // Recursion guard for near objects (cracks)
    if ray_bounce == 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(record) = world.hit(&ray, 0.001, f64::INFINITY) {
        let n = record.normal;
        let p = record.p;
        // s = diffuse target from P: (S - P)
        // TODO: parameterize diffusing methods
        // let s = p + n + Vec3::random_in_unit_sphere().normalize();
        let s = p + Vec3::random_in_hemisphere(&n);
        let diffuse_ray = Ray::new(p, s - p);
        return 0.5 * ray_color(diffuse_ray, world, ray_bounce - 1);
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_color(file: &mut File, color: &mut Color, samples_per_pixel: u8) -> std::io::Result<()> {
    // Divide the color by the number of samples to get the average
    *color /= samples_per_pixel as f64;
    // Gamma-correct for gamma=2.0.
    color.sqrt();
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

    // Don't run the program rls!
    if cfg!(debug_assertions) {
        return Ok(());
    }

    let mut file = File::create("image_hemisphere.ppm")?;

    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add(Arc::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    // Image
    let aspect_ratio: f64 = 16. / 9.;
    let image_width: u16 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u16;
    let samples_per_pixel = 100;
    let max_ray_bounce_depth = 50;

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
            let mut pixel_color =
                (0..samples_per_pixel)
                    .into_iter()
                    .fold(Color::new(0., 0., 0.), |acc, _| {
                        let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                        let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;
                        let ray = cam.get_ray(u, v);
                        acc + ray_color(ray, &world, max_ray_bounce_depth)
                    });
            write_color(&mut file, &mut pixel_color, samples_per_pixel)?;
        }
    }
    file.flush()?;

    Ok(())
}
