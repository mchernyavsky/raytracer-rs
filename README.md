# raytracer-rs

A ray tracer implemented in Rust, based on [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley.

## Features

- **Materials**: Lambertian (diffuse) and Metal (reflective with configurable fuzziness)
- **Geometry**: Sphere primitives with a hit-list scene graph
- **Camera**: Configurable via lower-left corner, horizontal/vertical extents, and origin
- **Anti-aliasing**: Multi-sample per pixel (100 samples by default)
- **Gamma correction**: Square-root gamma applied during color output
- **Parallel rendering**: Per-pixel parallelism via [rayon](https://github.com/rayon-rs/rayon)
- **Output**: PPM image format, written to stdout or a file

## Usage

### Build

```sh
cargo build --release
```

### Run

Output to stdout (pipe into a PPM viewer or redirect to a file):

```sh
cargo run --release > output.ppm
```

Output directly to a file:

```sh
cargo run --release output.ppm
```

## Scene

The default scene renders four spheres:

| Position       | Radius | Material              |
|----------------|--------|-----------------------|
| (0, 0, −1)     | 0.5    | Lambertian (red)      |
| (0, −100.5, −1)| 100    | Lambertian (yellow)   |
| (1, 0, −1)     | 0.5    | Metal (gold, fuzz 0.3)|
| (−1, 0, −1)    | 0.5    | Metal (silver, fuzz 1)|

Image size: **200 × 100** pixels.

## Project Structure

```
src/
  main.rs      – Entry point: scene setup, render loop, output
  lib.rs       – Public re-exports
  camera.rs    – Camera and ray generation
  ray.rs       – Ray type
  vec3.rs      – 3D vector math
  color.rs     – RGB color type
  image.rs     – Image buffer and PPM writer
  hit.rs       – Hit trait, HitRecord, HitList
  sphere.rs    – Sphere primitive
  material.rs  – Scatter trait, Lambertian and Metal materials
```

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| rand  | 0.6.5   | Random sampling for AA and diffuse scatter |
| rayon | 1.0.3   | Data-parallel pixel rendering |
