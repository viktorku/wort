#![allow(clippy::needless_return)]

use std::{io::prelude::*, time::Instant};

use rand::random;
use rayon::prelude::*;

mod core;
use crate::core::{color::Color, material::DiffuseMethod};

mod sinks;
use sinks::Sink;

mod arg;
use arg::{parse_arguments, Args};

mod scene;
use scene::{generate_scene, random_scene, get_camera};

// Image
const ASPECT_RATIO: f64 = 3. / 2.;
const IMAGE_WIDTH: usize = 600;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 500;
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
        mut diffuse_method,
        verbose,
    } = parse_arguments();

    // TODO: make arg into a param struct
    let mut trace = |diffuse_method: &mut DiffuseMethod| -> std::io::Result<std::vec::Vec<_>> {
        let start = Instant::now();

        let world = random_scene(diffuse_method);
        let camera = get_camera();

        // Render
        let mut pixels: Vec<Color> = Vec::with_capacity(IMAGE_WIDTH * IMAGE_HEIGHT);
        for j in (0..IMAGE_HEIGHT).rev() {
            if verbose {
                eprintln!("Scanlines remaining: {}", j);
                stdout.flush()?;
            }

            let par_iter = (0..IMAGE_WIDTH).into_par_iter().map(|i| {
                let mut pixel_color =
                    (0..SAMPLES_PER_PIXEL)
                        .into_iter()
                        .fold(Color::new(0., 0., 0.), |acc, _| {
                            let u = (i as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                            let v = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                            let ray = camera.get_ray(u, v);
                            acc + ray.color(&world, MAX_RAY_BOUNCE_DEPTH)
                        });
                // Divide the color by the number of samples to get the average
                pixel_color /= SAMPLES_PER_PIXEL as f64;
                // Gamma-correct for gamma=2.0.
                pixel_color.sqrt()
            });

            let mut line_pixels: Vec<_> = par_iter.collect();
            pixels.append(&mut line_pixels);
        }

        let duration = start.elapsed();
        eprintln!("Ray tracing took {:.3}s", duration.as_secs_f64());
        Ok(pixels)
    };

    match output {
        Sink::File => {
            let pixels = trace(&mut diffuse_method).unwrap();
            sinks::file::write_to_file(filename.unwrap(), &pixels)
        }
        Sink::Window => sinks::window::draw_in_window(trace, &mut diffuse_method),
    }
}
