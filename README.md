# raytracer
A ray tracer implemented in Rust based on "Ray Tracing in One Weekend"

## Usage

Requires a nightly Rust toolchain (the crate uses the unstable `try_blocks` feature).

Render the scene to a file:

```sh
cargo run --release -- image.ppm
```

With no arguments the PPM image is written to standard output:

```sh
cargo run --release > image.ppm
```

The output is in [PPM](https://en.wikipedia.org/wiki/Netpbm#File_formats) format
and can be viewed or converted with most image tools, e.g. `convert image.ppm image.png`.
