# WORT - a Week(end) of Ray Tracing

This is yet another implemention of the [Ray Tracing in One Weekend](https://raytracing.github.io/) tutorial/book. The original implementation is in C++ so I translated the code in a hopefully idiomatic Rust, which I also learned in parallel.

## Some extra features:

- Usage of simple framebuffer window ([`minifb`](https://crates.io/crates/minifb)) to draw the image and experiment with different diffuse methods via key presses
- Single threaded C++ implementation by Shirley is slightly faster (15.8s with either `-02` or `-03` optimizations) than single threaded Rust (18.6s) on the final random scene
- Rayon parallelizes tracing the ray along the X/width axis and gained 3.7-4 times speed up (5.3s)
- Rust lacks reflection so string-enum mapping and iterating had to be done via a custom crate [`strum`](https://crates.io/crates/strum)

## Running notes:

- Don't bother running the tracer in debug mode, as it's painfully slow
- Image output can be a PPM file or a framebuffer window
- There are a couple CLI arguments, run `--help` to see or check the code out

## TODO:

- Reported rendering time above is on 2015 MacBook Pro, but add times for i9-9900K (should be ~4 times faster?), and M1 silicon for an interesting comparison
- There are many parameters (such as camera positioning, dielectrics reflecitivity, object poeses, etc.) that can be made configurable via the framebuffer window or the CLI
- Implement the second and third part
- Play around with AVX/SIMD?
- ...
