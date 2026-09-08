# raytracer

A ray tracer implemented in Rust, based on Peter Shirley's
[*Ray Tracing in One Weekend*](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

It renders a fixed scene — four spheres on a large ground sphere, lit by a sky gradient — and
writes the result as an ASCII PPM (P3) image.

## Features

- Antialiasing by averaging multiple jittered samples per pixel
- Diffuse (Lambertian) and metal materials, with adjustable fuzz on reflections
- Recursive ray bounces with a depth limit, and gamma-2 correction on output
- Multithreaded rendering: pixels are computed in parallel with [rayon](https://github.com/rayon-rs/rayon)
- Scenes composed through the `Hit` trait, so hittable lists nest as ordinary hittables

## Requirements

The binary uses the unstable `try_blocks` feature, so **a nightly Rust toolchain is required**:

```bash
rustup toolchain install nightly
```

The library part of the crate compiles on stable.

## Building and running

```bash
cargo +nightly build --release
cargo +nightly run --release -- image.ppm    # write to a file
cargo +nightly run --release > image.ppm     # or to stdout
```

Use `--release`; a debug build renders very slowly. PPM files open in most image viewers, and can
be converted with e.g. `convert image.ppm image.png` (ImageMagick).

## Configuration

There is no command-line configuration beyond the optional output path. Resolution and quality are
compile-time constants in `src/main.rs`:

| Constant | Meaning |
| --- | --- |
| `IMAGE_WIDTH`, `IMAGE_HEIGHT` | Output resolution in pixels |
| `N_SAMPLES` | Rays per pixel (antialiasing quality) |
| `MAX_DEPTH` | Maximum number of ray bounces |

The scene itself is built in `make_world`, and the view in `make_camera`, both in `src/main.rs`.

## Project layout

| Module | Contents |
| --- | --- |
| `vec3.rs` | 3-component `f64` vector used for points, directions and linear colors |
| `ray.rs` | Ray as origin + direction |
| `camera.rs` | Maps normalized screen coordinates to rays |
| `hit.rs` | `Hit` trait, `HitRecord`, and `HitList` for composing scenes |
| `sphere.rs` | Sphere–ray intersection |
| `material.rs` | `Scatter` trait with `Lambertian` and `Metal` |
| `image.rs` | Pixel buffer, parallel pixel iterator, and PPM writer |
| `color.rs` | 8-bit RGB output color |

## Tests

Unit tests live alongside the code they cover and run on stable:

```bash
cargo test
cargo test --lib vec3::tests    # a single module
```

## License

MIT — see [LICENSE](LICENSE).
