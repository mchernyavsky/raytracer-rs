# Contributing

- Requires a nightly Rust toolchain (`src/main.rs` uses `#![feature(try_blocks)]`): `rustup toolchain install nightly`.
- Build: `cargo +nightly build` (use `--release` for renders — debug builds are very slow).
- Test: `cargo +nightly test` runs the unit tests in `src/vec3.rs` and `src/ray.rs`.
- Run: `cargo +nightly run --release -- out.ppm`, or omit the argument to write the PPM to stdout.
- Before opening a PR, make sure `cargo +nightly fmt` and `cargo +nightly clippy` are clean and tests pass.
