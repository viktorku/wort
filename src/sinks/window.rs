use crate::core::{color::Color, material::DiffuseMethod};
use crate::{IMAGE_HEIGHT, IMAGE_WIDTH};
use enum_iterator::IntoEnumIterator;
use minifb::{Key, Window, WindowOptions};

pub fn draw_in_window<F>(mut trace: F, diffuse_method: &mut DiffuseMethod) -> std::io::Result<()>
where
    F: FnMut(&mut DiffuseMethod) -> std::io::Result<std::vec::Vec<Color>>,
{
    let mut pixels = trace(diffuse_method).unwrap();

    let mut buffer: Vec<u32> = vec![0; IMAGE_WIDTH * IMAGE_HEIGHT];

    let mut window = Window::new(
        "ESC to exit",
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        WindowOptions {
            topmost: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut diffuse_method_iter = DiffuseMethod::into_enum_iter().cycle();
    eprintln!("Using {} diffuse method.", diffuse_method.to_string());

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, pixel) in buffer.iter_mut().zip(&pixels) {
            let c_u8 = pixel.as_u8_slice();
            // ARGB but alpha ignored
            *i = u32::from_be_bytes([0, c_u8[0], c_u8[1], c_u8[2]]);
        }

        if window.is_key_down(Key::D) {
            let next_diffuse_method = &mut diffuse_method_iter.next().unwrap();
            pixels = trace(next_diffuse_method).unwrap();
            eprintln!("Using {} diffuse method.", next_diffuse_method.to_string());
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, IMAGE_WIDTH, IMAGE_HEIGHT)
            .unwrap();
    }

    Ok(())
}
