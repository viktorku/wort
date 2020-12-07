use std::fs::File;
use std::io::prelude::*;

mod vec3;
use vec3::{Color, Point3, Vec3};

mod ray;
use ray::Ray;

const U8_MULTIPLIER: f32 = 255.999;

fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_color(file: &mut File, color: &mut Color) -> std::io::Result<()> {
    *color *= U8_MULTIPLIER;
    writeln!(
        file,
        "{} {} {}",
        color.x as u8, color.y as u8, color.z as u8
    )?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    let mut file = File::create("image.ppm")?;

    // Image
    let aspect_ratio: f32 = 16. / 9.;
    let image_width: u16 = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u16;

    // Camera
    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    // Render

    writeln!(file, "P3")?;
    writeln!(file, "{} {}", image_width, image_height)?;
    writeln!(file, "255")?;

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        stdout.flush()?;

        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let mut pixel_color = ray_color(ray);

            write_color(&mut file, &mut pixel_color)?;
        }
    }
    file.flush()?;

    Ok(())
}
