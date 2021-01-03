use std::{fs::File, path::Path, io::prelude::*, time::Instant};

use crate::core::color::Color;
use crate::{IMAGE_HEIGHT, IMAGE_WIDTH};

pub fn write_to_file(filename: String, pixels: &[Color]) -> std::io::Result<()> {
    let path = Path::new("renders").join("staging");
    std::fs::create_dir_all(&path)?;
    eprintln!("Writing to {}", filename);
    let mut file = File::create(path.join(filename))?;

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
