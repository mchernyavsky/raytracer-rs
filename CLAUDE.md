# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

A ray tracer in Rust following "Ray Tracing in One Weekend". The repo is developed as a series of
assignments (see git history), so each commit tends to add one chapter's worth of capability.

## Toolchain

`src/main.rs` starts with `#![feature(try_blocks)]`, so **the binary requires a nightly compiler**.
There is no `rust-toolchain` file, so nightly must be selected explicitly:

```bash
cargo +nightly build
cargo +nightly run > image.ppm     # render to stdout
cargo +nightly run -- image.ppm    # render to a file (single optional arg)
cargo +nightly run --release       # rendering is slow in debug builds
```

The library half (`src/lib.rs` and its modules) does not use nightly features, so `cargo test`,
`cargo check --lib`, and `cargo clippy --lib` work on stable; anything that compiles the binary
needs `+nightly`.

```bash
cargo test                          # all tests
cargo test --lib vec3::tests        # one module's tests
cargo test test_point_at_parameter  # one test by name
cargo fmt
cargo clippy
```

Unit tests live in `#[cfg(test)] mod tests` blocks inside the module they cover (currently
`ray.rs` and `vec3.rs`); there is no `tests/` directory.

## Architecture

The crate is both a library and a binary. `lib.rs` declares every module private and re-exports the
public surface from the crate root, so modules refer to each other as `crate::{Ray, Vec3}` rather
than by module path. New modules should follow that pattern: `mod x;` plus an explicit `pub use`.

The rendering pipeline, all wired together in `main.rs::draw_sphere`:

1. `Camera` maps `(u, v)` in [0,1] to a `Ray` via lower-left-corner + horizontal/vertical spans.
2. `Image` owns a flat `Box<[Color]>` buffer. `Image::pixels()` returns a **rayon** parallel
   iterator, so per-pixel work is multithreaded — hence `Hit` and `Scatter` are both `Send + Sync`.
3. `calc_pixel` jitters `N_SAMPLES` rays per pixel for antialiasing and averages them.
4. `color_vec_at` recurses: query the world for a hit, ask the hit's material to scatter, multiply
   by the scattered attenuation, and recurse until `MAX_DEPTH` or a miss (which returns the
   background sky gradient).
5. `write_ppm` serializes the buffer as ASCII PPM (P3), emitting rows bottom-up so `y` increases
   upward.

Two trait-object seams carry the extensibility:

- `Hit` (`hit.rs`) — geometry. `Sphere` implements it; `HitList` also implements it and holds
  `Vec<Box<dyn Hit>>`, so a scene is just another `Hit` and nesting composes for free. `hit()`
  returns `Option<HitRecord>` and `HitList` narrows `t_max` to `closest_so_far` while iterating to
  keep the nearest intersection.
- `Scatter` (`material.rs`) — materials (`Lambertian`, `Metal`). Returns `Option<ScatteredRay>`;
  `None` means the ray is absorbed. Spheres hold their material as `Arc<dyn Scatter>` so it can be
  shared across objects and threads, while `HitRecord` borrows it as `&'a dyn Scatter`.

`Vec3` (`vec3.rs`) is the `f64` math type used for points, directions, **and** linear colors; the
separate `Color` struct (`color.rs`) is only the `u8` RGB output representation. Conversion happens
in `vec_to_color`, which applies gamma-2 correction (`sqrt`) before scaling to bytes.

Numeric conventions worth preserving: shadow acne is avoided with `t_min = 0.001`; `Sphere::hit`
uses the half-`b` simplification of the quadratic, so `b = oc·dir` (not `2 * oc·dir`) and the
discriminant is `b*b - a*c`.

## Conventions

Fields are private with getter methods named after the field (`ray.origin()`, `hit.t()`); small
`Copy` types (`Vec3`, `Ray`, `Color`) take `self` by value in accessors. `main.rs` nests helper
functions inside the function that uses them, after an early `return`, and keeps tuning constants
(`IMAGE_WIDTH`, `N_SAMPLES`, `MAX_DEPTH`) as `const` at the narrowest scope that needs them.
