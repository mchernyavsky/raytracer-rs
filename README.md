# raytracer-rs

A ray tracer implemented in Rust, based on [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley.

## Features
1
2
3
4

- Diffuse (Lambertian) and metallic materials with configurable fuzz
- Anti-aliasing via multi-sample averaging (100 samples per pixel)
- Parallel rendering using [rayon](https://github.com/rayon-rs/rayon)
- PPM image output

## Requirements

- Rust nightly (uses `#![feature(try_blocks)]`)

## Build

```sh
cargo build --release
```

## Usage

Render to stdout:

```sh
cargo run --release > output.ppm
```

Render to a file:

```sh
cargo run --release output.ppm
```

## Scene

The default scene renders a 200×100 image with four spheres:

- A red Lambertian sphere in the center
- A large yellow Lambertian ground plane
- A gold Metal sphere on the right (fuzz 0.3)
- A silver Metal sphere on the left (fuzz 1.0)

## License

See [LICENSE](LICENSE).
