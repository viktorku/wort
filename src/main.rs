#![allow(clippy::needless_return)]

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

use clap::{App, Arg};
use num::clamp;
use rand::random;
use strum::VariantNames;

mod vec3;
use vec3::{Color, Vec3, Point3};

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

mod material;
use material::{DiffuseMethod, Lambertian, Metal};

const BLACK: Color = Color::new(0., 0., 0.);

fn ray_color(ray: Ray, world: &dyn Hittable, ray_bounce: u8) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    // Recursion guard for near objects (cracks)
    if ray_bounce == 0 {
        return BLACK;
    }

    if let Some(record) = world.hit(&ray, 0.001, f64::INFINITY) {
        if let Some(scatter) = record.material.scatter(&ray, &record) {
            return scatter.attenuation * ray_color(scatter.ray, world, ray_bounce - 1);
        }
        return BLACK;
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_color(file: &mut File, color: &mut Color, samples_per_pixel: u8) -> std::io::Result<()> {
    // Divide the color by the number of samples to get the average
    *color /= samples_per_pixel as f64;
    // Gamma-correct for gamma=2.0.
    *color = color.sqrt();
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

    // Argument parsing
    let matches = App::new("wort")
        .version("0.1")
        .author("Viktor K. <viktor@kunovski.com>")
        .about("a week(end) of ray tracing")
        .arg(
            Arg::with_name("diffuse")
                .short("d")
                .long("diffuse")
                .value_name("DIFFUSE")
                .help("Diffusing method")
                .takes_value(true)
                .possible_values(&DiffuseMethod::VARIANTS),
        )
        .arg(
            Arg::with_name("filename")
                .short("n")
                .long("filename")
                .value_name("FILE")
                .help("Filename - defaults to `image`")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .help("Verbosity, prints remaining scanline"),
        )
        .get_matches();

    let diffuse_default = DiffuseMethod::Hemisphere.into();
    let diffuse_str = matches.value_of("diffuse").unwrap_or(diffuse_default);
    let diffuse_method = DiffuseMethod::from_str(diffuse_str).unwrap();

    let filename = format!(
        "{}_{}.ppm",
        matches.value_of("filename").unwrap_or("image"),
        diffuse_str
    );
    let verbose = matches.is_present("verbose");

    eprintln!("Writing to {}", filename);
    let mut file = File::create(filename)?;

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
        if verbose {
            eprintln!("Scanlines remaining: {}", j);
        }
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
