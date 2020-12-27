#![allow(clippy::needless_return)]

use std::{io::prelude::*, sync::Arc};

use num::clamp;
use rand::random;
// use rayon::prelude::*;

mod core;
use crate::core::{
    camera::Camera,
    hittable_list::HittableList,
    material::{Lambertian, Metal},
    sphere::Sphere,
    vec3::{Color, ColorU32, Point3},
};

mod sinks;
use sinks::Sink;

mod arg;
use arg::{parse_arguments, Args};

// Image
const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_RAY_BOUNCE_DEPTH: usize = 50;

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();

    // Don't run the program rls!
    if cfg!(debug_assertions) {
        return Ok(());
    }

    // Argument parsing
    let Args {
        output,
        filename,
        diffuse_method,
        verbose,
    } = parse_arguments();

    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0), diffuse_method));
    let material_center = Arc::new(Lambertian::new(Color::new(0.4, 0.3, 0.6), diffuse_method));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    // Objects
    let planet = Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    ));
    let sphere_center = Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5, material_center));
    let sphere_left = Arc::new(Sphere::new(Point3::new(-1., 0.3, -1.5), 0.3, material_left));
    let sphere_right = Arc::new(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right));

    // World
    let mut world = HittableList::new();
    world.add(planet);
    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_right);

    // Camera
    let cam = Camera::new(ASPECT_RATIO);

    // Render
    let mut pixels: std::vec::Vec<ColorU32> = Vec::new();

    for j in (0..IMAGE_HEIGHT).rev() {
        if verbose {
            eprintln!("Scanlines remaining: {}", j);
            stdout.flush()?;
        }

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color =
                (0..SAMPLES_PER_PIXEL)
                    .into_iter()
                    .fold(Color::new(0., 0., 0.), |acc, _| {
                        let u = (i as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                        let v = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                        let ray = cam.get_ray(u, v);
                        acc + ray.color(&world, MAX_RAY_BOUNCE_DEPTH)
                    });

            // Divide the color by the number of samples to get the average
            pixel_color /= SAMPLES_PER_PIXEL as f64;
            // Gamma-correct for gamma=2.0.
            pixel_color = pixel_color.sqrt();

            pixels.push(ColorU32 {
                x: (255. * clamp(pixel_color.x, 0., 1.)) as u32,
                y: (255. * clamp(pixel_color.y, 0., 1.)) as u32,
                z: (255. * clamp(pixel_color.z, 0., 1.)) as u32,
            });
        }
    }

    match output {
        Sink::File => sinks::file::write_to_file(filename.unwrap(), &pixels),
        Sink::Window => sinks::window::draw_in_window(&pixels),
    }
}
