use std::fs::File;
use std::io::prelude::*;

use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};
use crate::core::vec3::ColorU32;

pub fn write_to_file(filename: String, pixels: &[ColorU32]) -> std::io::Result<()> {
    eprintln!("Writing to {}", filename);
    let mut file = File::create(filename)?;

    writeln!(file, "P3")?;
    writeln!(file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(file, "255")?;
    for color in pixels {
        writeln!(
            file,
            "{} {} {}",
            color.x as u8,
            color.y as u8,
            color.z as u8,
        )?;
    }
    file.flush()?;
    Ok(())
}
