# Notes

## Summary

`raytracer` (v0.8.0, edition 2018) is a small Rust ray tracer following
*Ray Tracing in One Weekend*. It builds both a library (`src/lib.rs`) and a
binary (`src/main.rs`).

## Library modules

| Module | Exports | Purpose |
| --- | --- | --- |
| `vec3` | `Vec3` | 3D vector math |
| `ray` | `Ray` | origin + direction rays |
| `camera` | `Camera` | maps (u, v) to rays |
| `hit` | `Hit`, `HitList` | intersection trait and collection |
| `sphere` | `Sphere` | sphere primitive |
| `material` | `Lambertian`, `Metal`, `Scatter` | diffuse and metallic scattering |
| `color` | `Color`, `RED` | 8-bit RGB color |
| `image` | `Image`, `write_ppm` | pixel buffer and PPM output |

## Rendering

`draw_sphere` in `src/main.rs` is the render entry point. It builds a camera
and a four-sphere world (two Lambertian, two Metal), then fills an
200x100 `Image`: `calc_pixel` takes 100 jittered samples per pixel and
`color_vec_at` traces each ray recursively up to a depth of 50, falling back
to a vertical sky gradient on a miss. Colors are gamma-corrected with a square
root before being written out as PPM.

## Dependencies

`rand` 0.6 for sampling and `rayon` 1.0 for parallel iteration.

## Note

`src/main.rs` starts with `#![feature(try_blocks)]`, so the binary requires a
nightly toolchain.
