# raytracer-rs

A ray tracer implemented in Rust, based on [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley.

## Features

- 3D vector math with full operator overloads
- Camera-based ray generation
- Ray–sphere intersection using the quadratic discriminant formula
- Two material types:
  - **Lambertian** – diffuse scattering via random unit sphere sampling
  - **Metal** – specular reflection with configurable fuzz/roughness
- Anti-aliasing via jittered supersampling (100 samples per pixel)
- Recursive ray tracing with configurable depth limit (max 50)
- Gamma correction on output colors
- Parallel pixel rendering using [Rayon](https://github.com/rayon-rs/rayon)
- PPM image output (file or stdout)

## Usage

```sh
cargo run --release > output.ppm
```

Or write to a file directly:

```sh
cargo run --release -- output.ppm
```

Open `output.ppm` with any PPM-compatible viewer (e.g. [GIMP](https://www.gimp.org/), `feh`, `eog`).

## Project Structure

| File | Description |
|------|-------------|
| `src/main.rs` | Entry point, scene setup, rendering pipeline |
| `src/vec3.rs` | `Vec3` – 3D vector with math operations |
| `src/ray.rs` | `Ray` – origin + direction, point-along-ray |
| `src/camera.rs` | `Camera` – viewport and ray generation |
| `src/sphere.rs` | `Sphere` – geometry and ray intersection |
| `src/hit.rs` | `Hit` trait, `HitRecord`, `HitList` |
| `src/material.rs` | `Scatter` trait, `Lambertian`, `Metal` |
| `src/color.rs` | `Color` – RGB representation |
| `src/image.rs` | `Image` – pixel buffer, PPM writer |

## Scene

The default scene (`make_world` in `main.rs`) contains four spheres:

- A small red diffuse sphere in the center
- A large yellow ground sphere
- A gold metallic sphere (left)
- A white metallic sphere (right)

## Dependencies

- [`rand 0.6`](https://crates.io/crates/rand) – random number generation for material scattering
- [`rayon 1.0`](https://crates.io/crates/rayon) – data-parallel pixel computation

## License

See [LICENSE](LICENSE).
