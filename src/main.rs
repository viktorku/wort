use std::fs::File;
use std::io::prelude::*;

mod vec3;
use vec3::{Color};

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;

const U8_MULTIPLIER: f32 = 255.999;

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

    writeln!(file, "P3")?;
    writeln!(file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(file, "255")?;

    let b = 0.25;

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        stdout.flush()?;

        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(
                i as f32 / (IMAGE_WIDTH - 1) as f32,
                j as f32 / (IMAGE_HEIGHT - 1) as f32,
                b,
            );
            write_color(&mut file, &mut color)?;
        }
    }
    file.flush()?;

    Ok(())
}
