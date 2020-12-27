use minifb::{Key, Window, WindowOptions};
use crate::{IMAGE_WIDTH, IMAGE_HEIGHT};
use crate::core::vec3::ColorU32;

pub fn draw_in_window(pixels: &[ColorU32]) -> std::io::Result<()> {
    let mut buffer: Vec<u32> = vec![0; IMAGE_WIDTH * IMAGE_HEIGHT];

    let mut window = Window::new(
        "ESC to exit",
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // eprintln!("buffer size: {}", buffer.len());
    // eprintln!("pixels size: {}", pixels.len());

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, pixel) in buffer.iter_mut().zip(pixels) {
            *i = (pixel.x << 16) | (pixel.y << 8) | pixel.z;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, IMAGE_WIDTH, IMAGE_HEIGHT)
            .unwrap();
    }

    Ok(())
}
