![version](https://img.shields.io/crates/v/rayimg)
![license](https://img.shields.io/crates/l/rayimg)
![build](https://img.shields.io/appveyor/build/ivan0sokin/rayimg)

# Ray image

## About

Renders image of some scene using ray tracing.\
This project is Rust adaption of [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) e-book by [Peter Shirley](https://github.com/petershirley)

## Usage

Add following to your ``Cargo.toml``:

```toml
[dependencies]
rayimg = "0.1.2"
```

## Renders

Rendered images from tests can be found in ``tests/output`` and in ``examples/output`` directories.

## Tests

To run tests type following in the shell:

```shell
cargo test --doc
cargo test --tests --release
cargo test --examples
```

Examples can be compiled and executed by typing next:

```shell
cargo run --examples (examples name or nothing to see available examples) --release
```

## Benchmarks

To run benchmarks you should install rustup nightly toolchain and then enter:
```shell
rustup run nightly cargo bench
```

## Documentation

You can find documentation at [docs.rs](https://docs.rs/rayimg).

## Dependencies

**[Rand](https://github.com/rust-random/rand)**

## License

[Ray image](https://github.com/ivan0sokin/rayimg) is licensed under the [Apache 2.0](LICENSE-APACHE)/[MIT](LICENSE-MIT) license.
