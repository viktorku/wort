use std::{fs::File, io::prelude::*, time::Instant};

use crate::core::color::Color;
use crate::{IMAGE_HEIGHT, IMAGE_WIDTH};

pub fn write_to_file(filename: String, pixels: &[Color]) -> std::io::Result<()> {
    eprintln!("Writing to {}", filename);
    let mut file = File::create(filename)?;

    let start = Instant::now();
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(file, "255")?;
    for color in pixels {
        let c_u8 = color.as_u8_slice();
        writeln!(file, "{} {} {}", c_u8[0], c_u8[1], c_u8[2],)?;
    }
    eprintln!("Writing to file took {:.3}s", start.elapsed().as_secs_f64());
    file.flush()?;
    Ok(())
}
